use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::utils::calc_quantity_and_volume;
use crate::{FundingRateMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "okx";

// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    instId: String,
    tradeId: String,
    px: String,
    sz: String,
    side: String,
    ts: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-order-book-channel
#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    asks: Vec<[String; 4]>,
    bids: Vec<[String; 4]>,
    ts: String,
    checksum: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-funding-rate-channel
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    instType: String,
    instId: String,
    fundingRate: String,
    nextFundingRate: String,
    fundingTime: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    arg: HashMap<String, String>,
    action: Option<String>, // snapshot, update, only applicable to order book
    data: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    if let Some(symbol) = ws_msg.arg.get("instId") {
        Ok(symbol.to_string())
    } else {
        Err(SimpleError::new("no instId found"))
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    let timestamp = ws_msg.data.iter().fold(std::i64::MIN, |a, x| {
        a.max(x["ts"].as_str().unwrap().parse::<i64>().unwrap())
    });

    if timestamp == std::i64::MIN {
        Err(SimpleError::new(format!("data is empty in {}", msg)))
    } else {
        Ok(Some(timestamp))
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        if !ws_msg.arg.contains_key("channel") {
            return MessageType::Other;
        }
        let channel = ws_msg.arg["channel"].clone();
        if channel == "trades" {
            MessageType::Trade
        } else if channel == "books-l2-tbt" {
            MessageType::L2Event
        } else if channel == "books5" {
            MessageType::L2TopK
        } else if channel == "tickers" {
            MessageType::BBO
        } else if channel.starts_with("candle") {
            MessageType::Candlestick
        } else if channel == "funding-rate" {
            MessageType::FundingRate
        } else {
            MessageType::Other
        }
    } else {
        MessageType::Other
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawTradeMsg>",
            msg
        ))
    })?;

    let mut trades: Vec<Result<TradeMsg, SimpleError>> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let timestamp = raw_trade.ts.parse::<i64>().unwrap();
            let price = raw_trade.px.parse::<f64>().unwrap();
            let size = raw_trade.sz.parse::<f64>().unwrap();
            let pair = crypto_pair::normalize_pair(&raw_trade.instId, EXCHANGE_NAME).unwrap();
            let (quantity_base, quantity_quote, _) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, size);

            Ok(TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instId.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base,
                quantity_quote,
                quantity_contract: if market_type == MarketType::Spot {
                    None
                } else {
                    Some(size)
                },
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.tradeId.clone(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            })
        })
        .collect();

    if trades.len() == 1 {
        if let Ok(v) = trades[0].as_mut() {
            v.json = msg.to_string();
        }
    }
    trades.into_iter().collect()
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawFundingRateMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawFundingRateMsg>",
            msg
        ))
    })?;

    let mut rates: Vec<FundingRateMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| {
            let pair = crypto_pair::normalize_pair(&raw_msg.instId, EXCHANGE_NAME).unwrap();
            FundingRateMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_msg.instId.clone(),
                pair,
                msg_type: MessageType::FundingRate,
                timestamp: Utc::now().timestamp_millis(),
                funding_rate: raw_msg.fundingRate.parse::<f64>().unwrap(),
                funding_time: raw_msg.fundingTime.parse::<i64>().unwrap(),
                estimated_rate: Some(raw_msg.nextFundingRate.parse::<f64>().unwrap()),
                json: serde_json::to_string(&raw_msg).unwrap(),
            }
        })
        .collect();

    if rates.len() == 1 {
        rates[0].json = msg.to_string();
    }
    Ok(rates)
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawOrderbookMsg>",
            msg
        ))
    })?;
    let snapshot = ws_msg.action.unwrap() == "snapshot";
    debug_assert_eq!(ws_msg.data.len(), 1);

    let symbol = ws_msg.arg["instId"].clone();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    let mut orderbooks = ws_msg
        .data
        .iter()
        .map(|raw_orderbook| {
            let timestamp = raw_orderbook.ts.parse::<i64>().unwrap();
            let parse_order = |raw_order: &[String; 4]| -> Order {
                let price = raw_order[0].parse::<f64>().unwrap();
                let quantity = raw_order[1].parse::<f64>().unwrap();
                let (quantity_base, quantity_quote, quantity_contract) =
                    calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

                Order {
                    price,
                    quantity_base,
                    quantity_quote,
                    quantity_contract,
                }
            };

            OrderBookMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.clone(),
                pair: pair.clone(),
                msg_type: MessageType::L2Event,
                timestamp,
                seq_id: None,
                prev_seq_id: None,
                asks: raw_orderbook
                    .asks
                    .iter()
                    .map(|x| parse_order(x))
                    .collect::<Vec<Order>>(),
                bids: raw_orderbook
                    .bids
                    .iter()
                    .map(|x| parse_order(x))
                    .collect::<Vec<Order>>(),
                snapshot,
                json: serde_json::to_string(raw_orderbook).unwrap(),
            }
        })
        .collect::<Vec<OrderBookMsg>>();

    if orderbooks.len() == 1 {
        orderbooks[0].json = msg.to_string();
    }
    Ok(orderbooks)
}
