use crypto_market_type::MarketType;

use super::utils::calc_quantity_and_volume;
use crate::{FundingRateMsg, MessageType, OrderBookMsg, TradeMsg, TradeSide};

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

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    data: Vec<T>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SwapTradeMsg>>(&msg)?;
    let trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let pair = crypto_pair::normalize_pair(&raw_trade.instrument_id, EXCHANGE_NAME).unwrap();
            let price = raw_trade.price.parse::<f64>().unwrap();
            let size = raw_trade.size.parse::<f64>().unwrap();
            let (quantity, volume) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, size);

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_id.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp.parse::<i64>().unwrap(),
                price,
                quantity,
                volume,
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                // Use timestamp as ID because bitget doesn't provide trade_id
                trade_id: raw_trade.timestamp.to_string(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

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

    let rates: Vec<FundingRateMsg> = ws_msg
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
            raw: serde_json::to_value(&raw_msg).unwrap(),
        })
        .collect();

    Ok(rates)
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    Ok(Vec::new())
}
