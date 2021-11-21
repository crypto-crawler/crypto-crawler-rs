use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::utils::calc_quantity_and_volume;
use crate::{FundingRateMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bitget";

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
    data: Vec<T>,
    action: Option<String>,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let instrument_ids = ws_msg
        .data
        .iter()
        .map(|v| v["instrument_id"].as_str().unwrap())
        .collect::<Vec<&str>>();
    if instrument_ids.is_empty() {
        None
    } else {
        Some(instrument_ids[0].to_string())
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SwapTradeMsg>>(msg)?;
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

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawFundingRateMsg>>(msg)?;

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

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SwapOrderbookMsg>>(msg)?;
    let snapshot = ws_msg.action.unwrap() == "partial";
    let mut orderbooks = Vec::<OrderBookMsg>::new();

    for raw_orderbook in ws_msg.data.iter() {
        let symbol = raw_orderbook.instrument_id.as_str();
        let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
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
            msg_type: MessageType::L2Event,
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
