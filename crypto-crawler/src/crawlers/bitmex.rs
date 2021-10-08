use super::{
    crawl_candlestick_ext, crawl_event,
    utils::{check_args, fetch_symbols_retry},
};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use std::sync::{Arc, Mutex};

const EXCHANGE_NAME: &str = "bitmex";

fn crawl_all(
    msg_type: MessageType,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
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
}

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::Trade, on_msg, duration)
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::Trade,
            market_type,
            symbols,
            on_msg,
            duration,
        );
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2Event, on_msg, duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::L2Event,
            market_type,
            symbols,
            on_msg,
            duration,
        );
    }
}

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::BBO, on_msg, duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::BBO,
            market_type,
            symbols,
            on_msg,
            duration,
        );
    }
}

pub(crate) fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2TopK, on_msg, duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::L2TopK,
            market_type,
            symbols,
            on_msg,
            duration,
        );
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
) {
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
    } else {
        crawl_candlestick_ext(
            EXCHANGE_NAME,
            market_type,
            symbol_interval_list,
            on_msg,
            duration,
        );
    }
}
