use crypto_market_type::MarketType;

use crate::{FundingRateMsg, MessageType, TradeMsg, TradeSide};

use super::super::utils::calc_quantity_and_volume;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "binance";

// see https://binance-docs.github.io/apidocs/spot/en/#aggregate-trade-streams
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct AggTradeMsg {
    e: String, // Event type
    E: i64,    // Event time
    s: String, // Symbol
    a: i64,    // Aggregate trade ID
    p: String, // Price
    q: String, // Quantity
    f: i64,    // First trade ID
    l: i64,    // Last trade ID
    T: i64,    // Trade time
    m: bool,   // Is the buyer the market maker?
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://binance-docs.github.io/apidocs/spot/en/#trade-streams
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    e: String, // Event type
    E: i64,    // Event time
    s: String, // Symbol
    t: i64,    // Trade ID
    p: String, // Price
    q: String, // Quantity
    b: i64,    // Buyer order ID
    a: i64,    // Seller order ID
    T: i64,    // Trade time
    m: bool,   // Is the buyer the market maker?
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    stream: String,
    data: T,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg)?;
    let data = obj.get("data").unwrap();
    let event_type = data.get("e").unwrap().as_str().unwrap();

    match event_type {
        "aggTrade" => {
            let agg_trade: AggTradeMsg = serde_json::from_value(data.clone()).unwrap();
            let mut trade = TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: agg_trade.s.clone(),
                pair: crypto_pair::normalize_pair(&agg_trade.s, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: agg_trade.T,
                price: agg_trade.p.parse::<f64>().unwrap(),
                quantity: agg_trade.q.parse::<f64>().unwrap(),
                volume: 0.0,
                side: if agg_trade.m {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: agg_trade.a.to_string(),
                raw: serde_json::from_str(msg)?,
            };
            let (quantity, volume) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &trade.pair,
                trade.price,
                trade.quantity,
            );
            trade.quantity = quantity;
            trade.volume = volume;
            Ok(vec![trade])
        }
        "trade" => {
            let raw_trade: RawTradeMsg = serde_json::from_value(data.clone()).unwrap();
            let mut trade = TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.s.clone(),
                pair: crypto_pair::normalize_pair(&raw_trade.s, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.T,
                price: raw_trade.p.parse::<f64>().unwrap(),
                quantity: raw_trade.q.parse::<f64>().unwrap(),
                volume: 0.0,
                side: if raw_trade.m {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.t.to_string(),
                raw: serde_json::from_str(msg)?,
            };
            let (quantity, volume) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &trade.pair,
                trade.price,
                trade.quantity,
            );
            trade.quantity = quantity;
            trade.volume = volume;
            Ok(vec![trade])
        }
        _ => panic!("Unsupported event type {}", event_type),
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    e: String,         // Event type
    E: i64,            // Event time
    s: String,         // Symbol
    p: String,         // Mark price
    i: Option<String>, // Index price
    P: String, // Estimated Settle Price, only useful in the last hour before the settlement starts
    r: String, // Funding rate
    T: i64,    // Next funding time
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg)?;
    let stream = obj.get("stream").unwrap().as_str().unwrap();
    let data = if stream == "!markPrice@arr" {
        obj.get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| serde_json::from_value::<RawFundingRateMsg>(x.clone()).unwrap())
            .collect()
    } else if stream.ends_with("@markPrice") {
        vec![serde_json::from_value::<RawFundingRateMsg>(obj.get("data").unwrap().clone()).unwrap()]
    } else {
        panic!("Unknown funding rate messaeg {}", msg);
    };
    let funding_rates: Vec<FundingRateMsg> = data
        .into_iter()
        .filter(|x| !x.r.is_empty())
        .map(|raw_msg| FundingRateMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol: raw_msg.s.clone(),
            pair: crypto_pair::normalize_pair(&raw_msg.s, EXCHANGE_NAME).unwrap(),
            msg_type: MessageType::FundingRate,
            timestamp: raw_msg.E,
            funding_rate: raw_msg.r.parse::<f64>().unwrap(),
            funding_time: raw_msg.T,
            estimated_rate: None,
            raw: if stream == "!markPrice@arr" {
                serde_json::to_value(&raw_msg).unwrap()
            } else {
                serde_json::from_str(msg).unwrap()
            },
        })
        .collect();
    Ok(funding_rates)
}
