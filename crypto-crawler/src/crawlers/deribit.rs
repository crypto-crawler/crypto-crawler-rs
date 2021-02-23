use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::{collections::HashMap, time::Duration};

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "deribit";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let channel = obj
        .get("params")
        .unwrap()
        .get("channel")
        .unwrap()
        .as_str()
        .unwrap();
    let first_dot_pos = channel.find('.').unwrap();
    let last_dot_pos = channel.rfind('.').unwrap();
    let symbol = &channel[(first_dot_pos + 1)..last_dot_pos];
    symbol.to_string()
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_internal, DeribitWSClient, MessageType::Trade, subscribe_trade);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if symbols.is_none() || symbols.unwrap().is_empty() {
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

        let channels: Vec<String> = match market_type {
            MarketType::InverseFuture => vec!["trades.future.BTC.raw", "trades.future.ETH.raw"],
            MarketType::InverseSwap => vec!["trades.BTC-PERPETUAL.raw", "trades.ETH-PERPETUAL.raw"],
            MarketType::Option => vec!["trades.option.BTC.raw", "trades.option.ETH.raw"],
            _ => panic!("Binance does NOT have the {} market type", market_type),
        }
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        let ws_client = DeribitWSClient::new(on_msg_ext, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
        None
    } else {
        crawl_trade_internal(market_type, symbols, on_msg, duration)
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, DeribitWSClient, MessageType::L2Event, subscribe_orderbook);
