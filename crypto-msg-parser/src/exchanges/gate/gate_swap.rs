use crypto_market_type::MarketType;

use super::super::utils::http_get;
use crate::{MessageType, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "gate";

lazy_static! {
    static ref QUANTO_MULTIPLIERS: HashMap<String, f64> = fetch_quanto_multipliers();
}

// get the quanto_multiplier field from:
// https://api.gateio.ws/api/v4/futures/usdt/contracts
// https://api.gateio.ws/api/v4/delivery/usdt/contracts
fn fetch_quanto_multipliers() -> HashMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    struct RawMarket {
        name: String,
        quanto_multiplier: String,
    }

    let mut mapping: HashMap<String, f64> = HashMap::new();

    let txt = http_get("https://api.gateio.ws/api/v4/futures/usdt/contracts").unwrap();
    let markets = serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap();
    for market in markets.iter() {
        let base = {
            let pos = market.name.find('_').unwrap();
            &market.name[..pos]
        };
        mapping.insert(
            base.to_string(),
            market.quanto_multiplier.parse::<f64>().unwrap(),
        );
    }

    let txt = http_get("https://api.gateio.ws/api/v4/delivery/usdt/contracts").unwrap();
    let markets = serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap();
    for market in markets.iter() {
        let base = {
            let dash_pos = market.name.find('_').unwrap();
            &market.name[..dash_pos]
        };
        mapping.insert(
            base.to_string(),
            market.quanto_multiplier.parse::<f64>().unwrap(),
        );
    }

    mapping
}

// https://www.gate.io/docs/delivery/ws/index.html#trades-subscription
#[derive(Serialize, Deserialize)]
struct FutureTradeMsg {
    size: f64,
    id: i64,
    create_time: i64,
    price: String,
    contract: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/futures/ws/index.html#trades-subscription
#[derive(Serialize, Deserialize)]
struct SwapTradeMsg {
    size: f64,
    id: i64,
    create_time: i64,
    create_time_ms: i64,
    price: String,
    contract: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    time: i64,
    channel: String,
    event: String,
    result: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn calc_quantity_and_volume(
    market_type: MarketType,
    pair: &str,
    price: f64,
    mut size: f64,
) -> (f64, f64) {
    size = f64::abs(size);

    match market_type {
        MarketType::InverseSwap => {
            // Each contract value is 1USD
            let volume = size;
            (volume / price, volume)
        }
        MarketType::LinearFuture | MarketType::LinearSwap => {
            let base = {
                let dash_pos = pair.find('/').unwrap();
                &pair[..dash_pos]
            };
            let real_quantity = size * QUANTO_MULTIPLIERS.get(base).unwrap();
            (real_quantity, real_quantity * price)
        }
        _ => panic!("Unknown market type {}", market_type),
    }
}

pub(super) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::LinearFuture => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<FutureTradeMsg>>(msg)?;

            let trades: Vec<TradeMsg> = ws_msg
                .result
                .into_iter()
                .map(|raw_trade| {
                    let symbol = raw_trade.contract.as_str();
                    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
                    let price = raw_trade.price.parse::<f64>().unwrap();
                    let (quantity, volume) =
                        calc_quantity_and_volume(market_type, &pair, price, raw_trade.size);

                    TradeMsg {
                        exchange: EXCHANGE_NAME.to_string(),
                        market_type,
                        symbol: symbol.to_string(),
                        pair,
                        msg_type: MessageType::Trade,
                        timestamp: raw_trade.create_time * 1000,
                        price,
                        quantity,
                        volume,
                        side: if raw_trade.size < 0.0 {
                            TradeSide::Sell
                        } else {
                            TradeSide::Buy
                        },
                        trade_id: raw_trade.id.to_string(),
                        raw: serde_json::to_value(&raw_trade).unwrap(),
                    }
                })
                .collect();

            Ok(trades)
        }
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<SwapTradeMsg>>(msg)?;

            let trades: Vec<TradeMsg> = ws_msg
                .result
                .into_iter()
                .map(|raw_trade| {
                    let symbol = raw_trade.contract.as_str();
                    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
                    let price = raw_trade.price.parse::<f64>().unwrap();
                    let (quantity, volume) =
                        calc_quantity_and_volume(market_type, &pair, price, raw_trade.size);

                    TradeMsg {
                        exchange: EXCHANGE_NAME.to_string(),
                        market_type,
                        symbol: symbol.to_string(),
                        pair,
                        msg_type: MessageType::Trade,
                        timestamp: raw_trade.create_time_ms,
                        price,
                        quantity,
                        volume,
                        side: if raw_trade.size < 0.0 {
                            TradeSide::Sell
                        } else {
                            TradeSide::Buy
                        },
                        trade_id: raw_trade.id.to_string(),
                        raw: serde_json::to_value(&raw_trade).unwrap(),
                    }
                })
                .collect();

            Ok(trades)
        }
        _ => panic!("Unknown market type {}", market_type),
    }
}
