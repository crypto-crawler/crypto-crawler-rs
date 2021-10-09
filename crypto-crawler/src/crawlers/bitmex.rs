use super::{
    crawl_candlestick_ext, crawl_event,
    utils::{check_args, fetch_symbols_retry},
};
use crate::{crawlers::utils::create_conversion_thread, msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "bitmex";

fn crawl_all(msg_type: MessageType, tx: Sender<Message>, duration: Option<u64>) {
    let tx = create_conversion_thread(EXCHANGE_NAME.to_string(), msg_type, MarketType::Unknown, tx);

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

    let ws_client = BitmexWSClient::new(tx, None);
    ws_client.subscribe(channels.as_slice());
    ws_client.run(duration);
}

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::Trade, tx, duration)
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::Trade,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2Event, tx, duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::L2Event,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::BBO, tx, duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::BBO,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}

pub(crate) fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2TopK, tx, duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::L2TopK,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::FundingRate, tx, duration);
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
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::FundingRate,
            market_type,
            tx,
        );

        let channels: Vec<String> = real_symbols
            .iter()
            .map(|symbol| format!("funding:{}", symbol))
            .collect();

        match market_type {
            MarketType::InverseSwap | MarketType::QuantoSwap => {
                let ws_client = BitmexWSClient::new(tx, None);
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
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Unknown {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Candlestick,
            market_type,
            tx,
        );

        let channels = vec!["tradeBin1m".to_string(), "tradeBin5m".to_string()];

        let ws_client = BitmexWSClient::new(tx, None);
        ws_client.subscribe(channels.as_slice());
        ws_client.run(duration);
    } else {
        crawl_candlestick_ext(
            EXCHANGE_NAME,
            market_type,
            symbol_interval_list,
            tx,
            duration,
        );
    }
}
