use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use super::utils::{
    check_args, fetch_symbols_retry, get_candlestick_intervals, get_connection_interval_ms,
    get_send_interval_ms,
};
use crate::utils::WS_LOCKS;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "bitmex";

// see <https://www.bitmex.com/app/wsAPI#Rate-Limits>
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 40;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_internal, BitmexWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_internal, BitmexWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_internal, BitmexWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_internal, BitmexWSClient, MessageType::L2TopK, subscribe_orderbook_topk);
#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick_internal, BitmexWSClient);

fn crawl_all(
    msg_type: MessageType,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            MarketType::Unknown,
            msg_type,
            msg,
        );
        (on_msg.lock().unwrap())(message);
    }));

    let channel: &str = match msg_type {
        MessageType::Trade => "trade",
        MessageType::L2Event => "orderBookL2_25",
        MessageType::L2TopK => "orderBook10",
        MessageType::BBO => "quote",
        MessageType::L2Snapshot => "orderBookL2",
        MessageType::FundingRate => "funding",
        _ => panic!("unsupported message type {}", msg_type),
    };
    let channels = vec![channel.to_string()];

    let ws_client = BitmexWSClient::new(on_msg_ext, None);
    ws_client.subscribe(channels.as_slice());
    ws_client.run(duration);
    None
}

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::Trade, on_msg, duration)
    } else {
        crawl_trade_internal(market_type, symbols, on_msg, duration)
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2Event, on_msg, duration)
    } else {
        crawl_l2_event_internal(market_type, symbols, on_msg, duration)
    }
}

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::BBO, on_msg, duration)
    } else {
        crawl_bbo_internal(market_type, symbols, on_msg, duration)
    }
}

pub(crate) fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2TopK, on_msg, duration)
    } else {
        crawl_l2_topk_internal(market_type, symbols, on_msg, duration)
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::FundingRate, on_msg, duration);
    } else {
        let is_empty = match symbols {
            Some(list) => {
                if list.is_empty() {
                    true
                } else {
                    check_args(EXCHANGE_NAME, market_type, list);
                    false
                }
            }
            None => true,
        };

        let real_symbols = if is_empty {
            fetch_symbols_retry(EXCHANGE_NAME, market_type)
        } else {
            symbols.unwrap().to_vec()
        };
        if real_symbols.is_empty() {
            panic!("real_symbols is empty");
        }
        let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                market_type,
                MessageType::FundingRate,
                msg,
            );
            (on_msg.lock().unwrap())(message);
        }));

        let channels: Vec<String> = real_symbols
            .iter()
            .map(|symbol| format!("funding:{}", symbol))
            .collect();

        match market_type {
            MarketType::InverseSwap | MarketType::QuantoSwap => {
                let ws_client = BitmexWSClient::new(on_msg_ext, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            _ => panic!("BitMEX {} does NOT have funding rates", market_type),
        }
    }
}

pub(crate) fn crawl_candlestick(
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::Unknown {
        let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                MarketType::Unknown,
                MessageType::Candlestick,
                msg,
            );
            (on_msg.lock().unwrap())(message);
        }));

        let channels = vec!["tradeBin1m".to_string(), "tradeBin5m".to_string()];

        let ws_client = BitmexWSClient::new(on_msg_ext, None);
        ws_client.subscribe(channels.as_slice());
        ws_client.run(duration);
        None
    } else {
        crawl_candlestick_internal(market_type, symbol_interval_list, on_msg, duration)
    }
}
