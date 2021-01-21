use std::time::{Duration, Instant};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use super::utils::fetch_symbols_retry;
use crate::{msg::Message, MessageType};
use crypto_markets::{get_market_types, MarketType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "bitfinex";
// All websocket connections have a limit of 30 subscriptions to public market data feed channels
// (tickers, book, candles, trades, …). We kindly ask all users to adapt their application setup
// accordingly to split subscriptions to channels using multiple WebSocket connections.
// see https://docs.bitfinex.com/docs/ws-general
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 30;

fn extract_symbol(json: &str) -> String {
    let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
    let obj = arr[0].as_object().unwrap();
    obj.get("symbol").unwrap().as_str().unwrap().to_string()
}

fn check_args(market_type: MarketType, symbols: &[String]) {
    let market_types = get_market_types(EXCHANGE_NAME);
    if !market_types.contains(&market_type) {
        panic!(
            "{} does NOT have the {} market type",
            EXCHANGE_NAME, market_type
        );
    }

    if symbols.len() > MAX_SUBSCRIPTIONS_PER_CONNECTION {
        error!("Each websocket connection has a limit of 30 subscriptions");
        panic!("Each websocket connection has a limit of 30 subscriptions");
    }

    let valid_symbols = fetch_symbols_retry(EXCHANGE_NAME, market_type);
    let invalid_symbols: Vec<String> = symbols
        .iter()
        .filter(|symbol| !valid_symbols.contains(symbol))
        .cloned()
        .collect();
    if !invalid_symbols.is_empty() {
        panic!(
            "Invalid symbols for {} {} market: {}, available trading symbols are {}",
            EXCHANGE_NAME,
            market_type,
            invalid_symbols.join(","),
            valid_symbols.join(",")
        );
    }
}

// #[rustfmt::skip]
// gen_crawl_event!(crawl_trade, BitfinexWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BitfinexWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, BitfinexWSClient, MessageType::L3Event, subscribe_l3_orderbook, true);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, MessageType::L2Snapshot, BitfinexRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l3_snapshot, MessageType::L3Snapshot, BitfinexRestClient::fetch_l3_snapshot);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    let is_empty = match symbols {
        Some(list) => {
            if list.is_empty() {
                true
            } else {
                check_args(market_type, &list);
                false
            }
        }
        None => true,
    };

    let real_symbols = if is_empty {
        fetch_symbols_retry(EXCHANGE_NAME, market_type)
    } else {
        symbols.unwrap().iter().cloned().collect::<Vec<String>>()
    };
    if real_symbols.is_empty() {
        error!("real_symbols is empty");
        panic!("real_symbols is empty");
    }

    let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            market_type,
            extract_symbol(&msg),
            MessageType::Trade,
            msg.to_string(),
        );
        (on_msg.lock().unwrap())(message);
    }));

    if real_symbols.len() <= MAX_SUBSCRIPTIONS_PER_CONNECTION {
        let should_stop = Arc::new(AtomicBool::new(false));
        let ws_client = Arc::new(BitfinexWSClient::new(on_msg_ext, None));

        if symbols.is_none() {
            let should_stop2 = should_stop.clone();
            let ws_client2 = ws_client.clone();

            std::thread::spawn(move || {
                while !should_stop2.load(Ordering::Acquire) {
                    let symbols = fetch_symbols_retry(EXCHANGE_NAME, market_type);
                    ws_client2.subscribe_trade(&symbols);
                    // update symbols every hour
                    std::thread::sleep(Duration::from_secs(3600));
                }
            });
        }

        ws_client.subscribe_trade(&real_symbols);
        ws_client.run(duration);
        should_stop.store(true, Ordering::Release);
    } else {
        // split to chunks
        let mut chunks: Vec<Vec<String>> = Vec::new();
        for i in (0..real_symbols.len()).step_by(MAX_SUBSCRIPTIONS_PER_CONNECTION) {
            let chunk = (&real_symbols
                [i..(std::cmp::min(i + MAX_SUBSCRIPTIONS_PER_CONNECTION, real_symbols.len()))])
                .iter()
                .cloned()
                .collect();
            chunks.push(chunk);
        }
        assert!(chunks.len() > 1);
        assert!(real_symbols.len() % MAX_SUBSCRIPTIONS_PER_CONNECTION != 0);

        if symbols.is_none() {
            let num_threads = Arc::new(AtomicUsize::new(chunks.len()));
            let last_client = Arc::new(BitfinexWSClient::new(on_msg_ext.clone(), None));

            for chunk in chunks.into_iter() {
                let on_msg_ext_clone = on_msg_ext.clone();
                let num_threads_clone = num_threads.clone();
                let last_client_clone = last_client.clone();
                std::thread::spawn(move || {
                    if chunk.len() < MAX_SUBSCRIPTIONS_PER_CONNECTION {
                        last_client_clone.subscribe_trade(&chunk);
                        last_client_clone.run(duration);
                    } else {
                        let ws_client = BitfinexWSClient::new(on_msg_ext_clone, None);
                        ws_client.subscribe_trade(&chunk);
                        ws_client.run(duration);
                    }

                    num_threads_clone.fetch_sub(1, Ordering::SeqCst);
                });
            }

            let mut subscribed_symbols = real_symbols.clone();
            while num_threads.load(Ordering::Acquire) > 0 {
                let latest_symbols = fetch_symbols_retry(EXCHANGE_NAME, market_type);
                let mut new_symbols: Vec<String> = latest_symbols
                    .iter()
                    .filter(|s| !subscribed_symbols.contains(s))
                    .cloned()
                    .collect();
                last_client.subscribe_trade(&new_symbols);
                subscribed_symbols.append(&mut new_symbols);
                // update symbols every hour
                std::thread::sleep(Duration::from_secs(duration.unwrap_or(3600)));
            }
        } else {
            let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

            for chunk in chunks.into_iter() {
                let on_msg_ext_clone = on_msg_ext.clone();
                let handle = std::thread::spawn(move || {
                    let ws_client = BitfinexWSClient::new(on_msg_ext_clone, None);
                    ws_client.subscribe_trade(&chunk);
                    ws_client.run(duration);
                });
                join_handles.push(handle);
            }
            for handle in join_handles {
                handle.join().unwrap();
            }
        }
    }
    None
}
