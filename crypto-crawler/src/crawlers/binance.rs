use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "Binance";

fn detect_symbol_market_type(is_contract: bool, symbol: &str) -> MarketType {
    if symbol.ends_with("_PERP") {
        MarketType::Swap
    } else if symbol.ends_with("-C") || symbol.ends_with("-P") {
        MarketType::Option
    } else if symbol.contains('_') {
        let date = &symbol[(symbol.len() - 6)..];
        debug_assert!(date.parse::<i64>().is_ok());
        MarketType::Future
    } else {
        if is_contract {
            MarketType::Swap
        } else {
            MarketType::Spot
        }
    }
}

fn check_args(market_type: MarketType, symbols: &[String]) {
    let is_contract = market_type != MarketType::Spot;
    let illegal_symbols: Vec<String> = symbols
        .iter()
        .filter(|symbol| detect_symbol_market_type(is_contract, symbol) != market_type)
        .map(|s| s.clone())
        .collect();
    if !illegal_symbols.is_empty() {
        panic!(
            "{} don't belong to {}",
            illegal_symbols.join(", "),
            market_type
        );
    }

    if market_type == MarketType::Swap {
        let linear_swap: Vec<String> = symbols
            .iter()
            .filter(|symbol| symbol.ends_with("USDT"))
            .map(|s| s.clone())
            .collect();
        let inverse_swap: Vec<String> = symbols
            .iter()
            .filter(|symbol| !symbol.ends_with("USDT"))
            .map(|s| s.clone())
            .collect();
        if linear_swap.len() > 0 && inverse_swap.len() > 0 {
            panic!("{} belong to linear swap, while {} belong to inverse swap, please split them into two lists", linear_swap.join(", "), inverse_swap.join(", "));
        }
    }
}

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    obj.get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("s")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, BinanceSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, BinanceSpotWSClient, MessageType::L2Event, subscribe_orderbook);

pub(crate) fn crawl_l2_snapshot<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let mut on_msg_ext = |json: String, symbol: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            MessageType::L2Snapshot,
            json,
        );
        on_msg(message);
    };

    let now = Instant::now();
    loop {
        let mut succeeded = false;
        for symbol in symbols.iter() {
            let resp = match market_type {
                MarketType::Spot => BinanceSpotRestClient::fetch_l2_snapshot(symbol),
                MarketType::Future => BinanceFutureRestClient::fetch_l2_snapshot(symbol),
                MarketType::Swap => {
                    if symbol.ends_with("USDT") {
                        BinanceLinearSwapRestClient::fetch_l2_snapshot(symbol)
                    } else {
                        BinanceInverseSwapRestClient::fetch_l2_snapshot(symbol)
                    }
                }
                MarketType::Option => BinanceOptionRestClient::fetch_l2_snapshot(symbol),
            };
            match resp {
                Ok(msg) => {
                    on_msg_ext(msg, symbol.to_string());
                    succeeded = true
                }
                Err(err) => error!(
                    "{} {} {}, error: {}",
                    EXCHANGE_NAME, market_type, symbol, err
                ),
            }
        }

        if let Some(seconds) = duration {
            if now.elapsed() > Duration::from_secs(seconds) && succeeded {
                break;
            }
        }

        std::thread::sleep(Duration::from_secs(crate::SNAPSHOT_INTERVAL));
    }
}
