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
    let symbol = symbol.to_lowercase();
    if symbol.ends_with("_perp") {
        MarketType::Swap
    } else if symbol.ends_with("-c") || symbol.ends_with("-p") {
        MarketType::Option
    } else if symbol.contains('_') {
        let date = &symbol[(symbol.len() - 6)..];
        debug_assert!(date.parse::<i64>().is_ok());
        MarketType::Future
    } else if is_contract {
        MarketType::Swap
    } else {
        MarketType::Spot
    }
}

fn check_args(market_type: MarketType, symbols: &[String]) {
    let is_contract = market_type != MarketType::Spot;
    let illegal_symbols: Vec<String> = symbols
        .iter()
        .filter(|symbol| detect_symbol_market_type(is_contract, symbol) != market_type)
        .cloned()
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
            .cloned()
            .collect();
        let inverse_swap: Vec<String> = symbols
            .iter()
            .filter(|symbol| !symbol.ends_with("USDT"))
            .cloned()
            .collect();
        if !linear_swap.is_empty() && !inverse_swap.is_empty() {
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
gen_crawl_event!(crawl_trade_spot, market_type, symbols, on_msg, duration, BinanceSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, market_type, symbols, on_msg, duration, BinanceSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_future, market_type, symbols, on_msg, duration, BinanceFutureWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_future, market_type, symbols, on_msg, duration, BinanceFutureWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, market_type, symbols, on_msg, duration, BinanceLinearSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, market_type, symbols, on_msg, duration, BinanceLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, market_type, symbols, on_msg, duration, BinanceInverseSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, market_type, symbols, on_msg, duration, BinanceInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_option, market_type, symbols, on_msg, duration, BinanceOptionWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_option, market_type, symbols, on_msg, duration, BinanceOptionWSClient, MessageType::L2Event, subscribe_orderbook);

pub(crate) fn crawl_trade<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let symbols: Vec<String> = symbols.iter().map(|symbol| symbol.to_lowercase()).collect();
    check_args(market_type, &symbols);

    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, &symbols, on_msg, duration),
        MarketType::Future => crawl_trade_future(market_type, &symbols, on_msg, duration),
        MarketType::Swap => {
            if symbols[0].ends_with("usdt") {
                crawl_trade_linear_swap(market_type, &symbols, on_msg, duration);
            } else {
                crawl_trade_inverse_swap(market_type, &symbols, on_msg, duration);
            }
        }
        MarketType::Option => {
            let symbols: Vec<String> = symbols.iter().map(|symbol| symbol.to_uppercase()).collect();
            crawl_trade_option(market_type, &symbols, on_msg, duration);
        }
    }
}

pub(crate) fn crawl_l2_event<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let symbols: Vec<String> = symbols.iter().map(|symbol| symbol.to_lowercase()).collect();
    check_args(market_type, &symbols);

    match market_type {
        MarketType::Spot => crawl_l2_event_spot(market_type, &symbols, on_msg, duration),
        MarketType::Future => crawl_l2_event_future(market_type, &symbols, on_msg, duration),
        MarketType::Swap => {
            if symbols[0].ends_with("usdt") {
                crawl_l2_event_linear_swap(market_type, &symbols, on_msg, duration);
            } else {
                crawl_l2_event_inverse_swap(market_type, &symbols, on_msg, duration);
            }
        }
        MarketType::Option => crawl_l2_event_option(market_type, &symbols, on_msg, duration),
    }
}

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

    let symbols: Vec<String> = symbols.iter().map(|symbol| symbol.to_uppercase()).collect();

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
