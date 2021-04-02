use crypto_market_type::MarketType;

use super::utils::http_get;
use crate::{FundingRateMsg, MessageType, TradeMsg, TradeSide};

use chrono::prelude::*;
use chrono::DateTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "okex";

lazy_static! {
    static ref LINEAR_FUTURE_CONTRACT_VALUE_MAP: HashMap<String, f64> =
        fetch_contract_val("futures");
    static ref LINEAR_SWAP_CONTRACT_VALUE_MAP: HashMap<String, f64> = fetch_contract_val("swap");
}

// get the contract_val field
// market_type, futures, swap, option
fn fetch_contract_val(market_type: &str) -> HashMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    struct Instrument {
        underlying: String,
        contract_val: String,
        is_inverse: String,
    }
    let mut mapping: HashMap<String, f64> = HashMap::new();

    let txt = http_get(&format!(
        "https://www.okex.com/api/{}/v3/instruments",
        market_type
    ))
    .unwrap();
    let instruments = serde_json::from_str::<Vec<Instrument>>(&txt).unwrap();

    for instrument in instruments.iter().filter(|x| x.is_inverse == "false") {
        let pair = instrument.underlying.replace('-', "/");
        mapping.insert(pair, instrument.contract_val.parse::<f64>().unwrap());
    }

    mapping
}

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

fn calc_quantity_and_volume(
    market_type: MarketType,
    pair: &str,
    price: f64,
    size: f64,
) -> (f64, f64) {
    match market_type {
        MarketType::Spot => (size, size * price),
        MarketType::LinearFuture => {
            let contract_val = LINEAR_FUTURE_CONTRACT_VALUE_MAP.get(pair).unwrap();
            let quantity = contract_val * size;
            (quantity, quantity * price)
        }
        MarketType::LinearSwap => {
            let contract_val = LINEAR_SWAP_CONTRACT_VALUE_MAP.get(pair).unwrap();
            let quantity = contract_val * size;
            (quantity, quantity * price)
        }
        MarketType::InverseFuture | MarketType::InverseSwap => {
            let contract_value = if pair.starts_with("BTC/") {
                100.0
            } else {
                10.0
            };
            let volume = contract_value * size;
            (volume / price, volume)
        }
        MarketType::Option => {
            let multiplier = match pair {
                "BTC/USD" => 0.1,
                "ETH/USD" => 1.0,
                "EOS/USD" => 100.0,
                _ => panic!("Unknown OKEx option pair {}", pair),
            };
            let quantity = size * multiplier;
            (quantity, quantity * price)
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
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
            let (quantity, volume) = calc_quantity_and_volume(market_type, &pair, price, size);

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
