use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{msg::Message, MarketType, MessageType};
use crypto_ws_client::{BinanceSpotWSClient, WSClient};
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

fn check_symbols(market_type: MarketType, symbols: &[String]) {
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

fn convert_to_message(json: String, market_type: MarketType, msg_type: MessageType) -> Message {
    Message {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: market_type,
        msg_type,
        symbol: extract_symbol(&json),
        received_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        json,
    }
}

pub(crate) fn crawl_trade<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_symbols(market_type, symbols);

    let on_msg_ext = |msg: String| {
        let message = convert_to_message(msg.to_string(), MarketType::Spot, MessageType::Trade);
        on_msg(message);
    };
    let mut ws_client = BinanceSpotWSClient::new(Box::new(on_msg_ext), None);
    ws_client.subscribe_trade(symbols);
    ws_client.run(duration);
}
