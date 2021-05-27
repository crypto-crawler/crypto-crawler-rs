use crypto_market_type::MarketType;

use super::super::utils::calc_quantity_and_volume;

use crate::{MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "gate";

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
                    let quantity = f64::abs(raw_trade.size);
                    let (quantity_base, quantity_quote, quantity_contract) =
                        calc_quantity_and_volume(
                            EXCHANGE_NAME,
                            market_type,
                            &pair,
                            price,
                            quantity,
                        );

                    TradeMsg {
                        exchange: EXCHANGE_NAME.to_string(),
                        market_type,
                        symbol: symbol.to_string(),
                        pair,
                        msg_type: MessageType::Trade,
                        timestamp: raw_trade.create_time * 1000,
                        price,
                        quantity_base,
                        quantity_quote,
                        quantity_contract,
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
                    let quantity = f64::abs(raw_trade.size);
                    let (quantity_base, quantity_quote, quantity_contract) =
                        calc_quantity_and_volume(
                            EXCHANGE_NAME,
                            market_type,
                            &pair,
                            price,
                            quantity,
                        );

                    TradeMsg {
                        exchange: EXCHANGE_NAME.to_string(),
                        market_type,
                        symbol: symbol.to_string(),
                        pair,
                        msg_type: MessageType::Trade,
                        timestamp: raw_trade.create_time_ms,
                        price,
                        quantity_base,
                        quantity_quote,
                        quantity_contract,
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
