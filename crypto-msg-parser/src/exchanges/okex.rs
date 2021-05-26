use crypto_market_type::MarketType;

use super::utils::calc_quantity_and_volume;
use crate::{FundingRateMsg, MessageType, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "okex";

// https://www.okex.com/docs/en/#spot_ws-trade
// https://www.okex.com/docs/en/#futures_ws-trade
// https://www.okex.com/docs/en/#ws_swap-trade
// https://www.okex.com/docs/en/#option_ws-trade
#[derive(Serialize, Deserialize)]
struct RawTradeMsg {
    instrument_id: String,
    trade_id: String,
    price: String,
    size: Option<String>,
    qty: Option<String>,
    trade_side: Option<String>, // buy, sell, for option/trades only
    side: Option<String>,       // buy, sell, for other
    timestamp: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RawFundingRateMsg {
    estimated_rate: String,
    funding_rate: String,
    funding_time: String,
    instrument_id: String,
    settlement_time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    data: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg)?;
    let option_trades = ws_msg.table.as_str() == "option/trades";
    let trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.timestamp).unwrap();
            let price = raw_trade.price.parse::<f64>().unwrap();
            let size = if raw_trade.qty.is_some() {
                raw_trade.qty.clone().unwrap().parse::<f64>().unwrap()
            } else if raw_trade.size.is_some() {
                raw_trade.size.clone().unwrap().parse::<f64>().unwrap()
            } else {
                panic!("qty and size are both missing");
            };
            let side = if option_trades {
                raw_trade.trade_side.clone().unwrap()
            } else {
                raw_trade.side.clone().unwrap()
            };
            let pair =
                crypto_pair::normalize_pair(&raw_trade.instrument_id, EXCHANGE_NAME).unwrap();
            let (quantity, volume) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, size);

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_id.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp: timestamp.timestamp_millis(),
                price,
                quantity,
                volume,
                side: if side.as_str() == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trade_id.to_string(),
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
            let funding_time = DateTime::parse_from_rfc3339(&raw_msg.funding_time).unwrap();
            FundingRateMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_msg.instrument_id.clone(),
                pair: crypto_pair::normalize_pair(&raw_msg.instrument_id, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::FundingRate,
                timestamp: Utc::now().timestamp_millis(),
                funding_rate: raw_msg.funding_rate.parse::<f64>().unwrap(),
                funding_time: funding_time.timestamp_millis(),
                estimated_rate: Some(raw_msg.estimated_rate.parse::<f64>().unwrap()),
                raw: serde_json::to_value(&raw_msg).unwrap(),
            }
        })
        .collect();

    Ok(rates)
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    Ok(Vec::new())
}
