use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::super::super::utils::calc_quantity_and_volume;
use crypto_message::{FundingRateMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use super::super::EXCHANGE_NAME;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

// see https://bitgetlimited.github.io/apidoc/en/swap/#public-trading-channel
#[derive(Serialize, Deserialize)]
struct SwapTradeMsg {
    instrument_id: String,
    price: String,
    side: String, // buy, sell
    size: String,
    timestamp: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://bitgetlimited.github.io/apidoc/en/swap/#public-market-depth-channel
#[derive(Serialize, Deserialize)]
struct SwapOrderbookMsg {
    instrument_id: String,
    timestamp: String,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    data: T,
    action: Option<String>,
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    if ws_msg.data.is_array() {
        let instrument_ids = ws_msg
            .data
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["instrument_id"].as_str().unwrap())
            .collect::<Vec<&str>>();
        if instrument_ids.is_empty() {
            Err(SimpleError::new(format!("data is empty {}", msg)))
        } else {
            Ok(instrument_ids[0].to_string())
        }
    } else if ws_msg.data.is_object() && ws_msg.data.get("instrument_id").is_some() {
        Ok(ws_msg.data["instrument_id"].as_str().unwrap().to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    let table = ws_msg.table.as_str();
    let timestamp = if table.starts_with("swap/candle") {
        Some(
            ws_msg.data["candle"].as_array().unwrap()[0]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        )
    } else {
        ws_msg
            .data
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["timestamp"].as_str().unwrap().parse::<i64>().unwrap())
            .max()
    };
    if timestamp.is_none() {
        Err(SimpleError::new(format!("data is empty in {}", msg)))
    } else {
        Ok(timestamp)
    }
}

pub(super) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let table = ws_msg.table;
        let channel = {
            let arr = table.split('/').collect::<Vec<&str>>();
            arr[1]
        };
        if channel == "trade" {
            MessageType::Trade
        } else if channel == "depth" {
            MessageType::L2Event
        } else if channel == "depth5" {
            MessageType::L2TopK
        } else if channel == "ticker" {
            MessageType::Ticker
        } else if channel.starts_with("candle") {
            MessageType::Candlestick
        } else if channel == "funding_rate" {
            MessageType::FundingRate
        } else {
            MessageType::Other
        }
    } else {
        MessageType::Other
    }
}

pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<SwapTradeMsg>>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SwapTradeMsg>",
            msg
        ))
    })?;
    let mut trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let pair =
                crypto_pair::normalize_pair(&raw_trade.instrument_id, EXCHANGE_NAME).unwrap();
            let price = raw_trade.price.parse::<f64>().unwrap();
            let size = raw_trade.size.parse::<f64>().unwrap();
            let (quantity_base, quantity_quote, quantity_contract) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, size);

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_id.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp.parse::<i64>().unwrap(),
                price,
                quantity_base,
                quantity_quote,
                quantity_contract,
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                // Use timestamp as ID because bitget doesn't provide trade_id
                trade_id: raw_trade.timestamp.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();
    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

// https://bitgetlimited.github.io/apidoc/en/swap/#public-fund-fees-channel
#[derive(Serialize, Deserialize)]
struct RawFundingRateMsg {
    funding_rate: String,
    funding_time: String,
    instrument_id: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<Vec<RawFundingRateMsg>>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<RawFundingRateMsg>",
                msg
            ))
        })?;

    let mut rates: Vec<FundingRateMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| FundingRateMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol: raw_msg.instrument_id.clone(),
            pair: crypto_pair::normalize_pair(&raw_msg.instrument_id, EXCHANGE_NAME).unwrap(),
            msg_type: MessageType::FundingRate,
            timestamp: Utc::now().timestamp_millis(),
            funding_rate: raw_msg.funding_rate.parse::<f64>().unwrap(),
            funding_time: raw_msg.funding_time.parse::<i64>().unwrap(),
            estimated_rate: None,
            json: serde_json::to_string(&raw_msg).unwrap(),
        })
        .collect();
    if rates.len() == 1 {
        rates[0].json = msg.to_string();
    }
    Ok(rates)
}

pub(super) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<Vec<SwapOrderbookMsg>>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<SwapOrderbookMsg>",
                msg
            ))
        })?;
    let table = ws_msg.table.as_str();

    let snapshot = if let Some(action) = ws_msg.action {
        action.as_str() == "partial"
    } else {
        table.starts_with("swap/depth") && table.chars().last().unwrap().is_numeric()
    };
    let msg_type = if table.starts_with("swap/depth") && table.chars().last().unwrap().is_numeric()
    {
        MessageType::L2TopK
    } else {
        MessageType::L2Event
    };

    let mut orderbooks = Vec::<OrderBookMsg>::new();

    for raw_orderbook in ws_msg.data.iter() {
        let symbol = raw_orderbook.instrument_id.as_str();
        let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg))
        })?;
        let timestamp = raw_orderbook.timestamp.parse::<i64>().unwrap();

        let parse_order = |raw_order: &[String; 2]| -> Order {
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

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol: symbol.to_string(),
            pair: pair.clone(),
            msg_type,
            timestamp,
            seq_id: None,
            prev_seq_id: None,
            asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
            bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
            snapshot,
            json: msg.to_string(),
        };

        orderbooks.push(orderbook)
    }

    Ok(orderbooks)
}
