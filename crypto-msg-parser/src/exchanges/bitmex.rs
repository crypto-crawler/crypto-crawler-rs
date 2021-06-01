use crypto_market_type::MarketType;

use crate::exchanges::utils::calc_quantity_and_volume;
use crate::Order;
use crate::{FundingRateMsg, MessageType, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use chrono::DateTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::sync::Mutex;

const EXCHANGE_NAME: &str = "bitmex";

// see https://www.bitmex.com/app/wsAPI#Response-Format
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy'
    size: f64,
    price: f64,
    tickDirection: String, // MinusTick, PlusTick, ZeroMinusTick, ZeroPlusTick
    trdMatchID: String,
    grossValue: f64,
    homeNotional: f64,
    foreignNotional: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawOrder {
    symbol: String,
    id: i64,
    side: String, // Sell, Buy
    size: Option<f64>,
    price: Option<f64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    timestamp: String,
    symbol: String,
    fundingInterval: String,
    fundingRate: f64,
    fundingRateDaily: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    action: String,
    data: Vec<T>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg)?;
    let raw_trades = ws_msg.data;
    let trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            // assert_eq!(raw_trade.foreignNotional, raw_trade.homeNotional * raw_trade.price); // tiny diff actually exists
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.timestamp).unwrap();

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.symbol.to_string(),
                pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: timestamp.timestamp_millis(),
                price: raw_trade.price,
                quantity_base: raw_trade.homeNotional,
                quantity_quote: raw_trade.foreignNotional,
                quantity_contract: Some(raw_trade.size),
                side: if raw_trade.side == "Sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trdMatchID.clone(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawFundingRateMsg>>(msg)?;
    let rates: Vec<FundingRateMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| {
            let settlement_time = DateTime::parse_from_rfc3339(&raw_msg.timestamp).unwrap();

            FundingRateMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_msg.symbol.clone(),
                pair: crypto_pair::normalize_pair(&raw_msg.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::FundingRate,
                timestamp: Utc::now().timestamp_millis(),
                funding_rate: raw_msg.fundingRate,
                funding_time: settlement_time.timestamp_millis(),
                estimated_rate: None,
                raw: serde_json::to_value(&raw_msg).unwrap(),
            }
        })
        .collect();

    Ok(rates)
}

lazy_static! {
    // id -> price
    static ref PRICE_HASHMAP: Mutex<HashMap<String,HashMap<i64, f64>>> = Mutex::new(HashMap::new());
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let mut price_map = PRICE_HASHMAP.lock().unwrap();

    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrder>>(msg)?;
    let snapshot = ws_msg.action == "partial";
    if ws_msg.data.is_empty() {
        return Ok(Vec::new());
    }
    let symbol = ws_msg.data[0].symbol.clone();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    if ws_msg.action == "insert" || ws_msg.action == "partial" {
        if !price_map.contains_key(&symbol) {
            price_map.insert(symbol.clone(), HashMap::new());
        }
        let symbol_price_map = price_map.get_mut(&symbol).unwrap();

        for x in ws_msg.data.iter() {
            symbol_price_map.insert(x.id, x.price.unwrap());
        }
    } else if !price_map.contains_key(&symbol) {
        panic!("{}", msg);
        // return Ok(Vec::new());
    }

    let symbol_price_map = price_map.get_mut(&symbol).unwrap();
    println!("{}", symbol_price_map.len());

    let parse_order = |raw_order: &RawOrder| -> Order {
        let price = if let Some(p) = raw_order.price {
            p
        } else {
            if !symbol_price_map.contains_key(&raw_order.id) {
                println!("{}", raw_order.id);
            }
            *symbol_price_map.get(&raw_order.id).unwrap()
        };

        let quantity = raw_order.size.unwrap_or(0.0); // 0.0 means delete
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
        vec![
            price,
            quantity_base,
            quantity_quote,
            quantity_contract.unwrap(),
        ]
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.clone(),
        msg_type: MessageType::L2Event,
        timestamp: Utc::now().timestamp_millis(),
        asks: ws_msg
            .data
            .iter()
            .filter(|x| x.side == "Sell")
            .map(|x| parse_order(x))
            .collect(),
        bids: ws_msg
            .data
            .iter()
            .filter(|x| x.side == "Buy")
            .map(|x| parse_order(x))
            .collect(),
        snapshot,
        raw: serde_json::from_str(msg)?,
    };

    if ws_msg.action == "delete" {
        for raw_order in ws_msg.data.iter() {
            symbol_price_map.remove(&raw_order.id);
        }
    }

    Ok(vec![orderbook])
}
