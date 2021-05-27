use crypto_market_type::MarketType;

use crate::{MessageType, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bybit";

// see https://bybit-exchange.github.io/docs/inverse/#t-websockettrade
#[derive(Serialize, Deserialize)]
struct InverseTradeMsg {
    trade_time_ms: i64,
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy
    size: f64,
    price: f64,
    tick_direction: String,
    trade_id: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://bybit-exchange.github.io/docs/linear/#t-websockettrade
#[derive(Serialize, Deserialize)]
struct LinearTradeMsg {
    trade_time_ms: String,
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy
    size: f64,
    price: String,
    tick_direction: String,
    trade_id: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    topic: String,
    data: Vec<T>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<InverseTradeMsg>>(msg)?;

            let trades: Vec<TradeMsg> = ws_msg
                .data
                .into_iter()
                .map(|raw_trade| TradeMsg {
                    exchange: EXCHANGE_NAME.to_string(),
                    market_type,
                    symbol: raw_trade.symbol.clone(),
                    pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
                    msg_type: MessageType::Trade,
                    timestamp: raw_trade.trade_time_ms,
                    price: raw_trade.price,
                    quantity_base: raw_trade.size / raw_trade.price,
                    // Each inverse contract value is 1 USD, see:
                    // https://www.bybit.com/data/basic/inverse/contract-detail?symbol=BTCUSD
                    // https://www.bybit.com/data/basic/future-inverse/contract-detail?symbol=BTCUSD0625
                    quantity_quote: raw_trade.size,
                    quantity_contract: Some(raw_trade.size),
                    side: if raw_trade.side == "Sell" {
                        TradeSide::Sell
                    } else {
                        TradeSide::Buy
                    },
                    trade_id: raw_trade.trade_id.clone(),
                    raw: serde_json::to_value(&raw_trade).unwrap(),
                })
                .collect();

            Ok(trades)
        }
        MarketType::LinearSwap => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<LinearTradeMsg>>(msg)?;

            let trades: Vec<TradeMsg> = ws_msg
                .data
                .into_iter()
                .map(|raw_trade| {
                    let price = raw_trade.price.parse::<f64>().unwrap();
                    TradeMsg {
                        exchange: EXCHANGE_NAME.to_string(),
                        market_type,
                        symbol: raw_trade.symbol.clone(),
                        pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME)
                            .unwrap(),
                        msg_type: MessageType::Trade,
                        timestamp: raw_trade.trade_time_ms.parse::<i64>().unwrap(),
                        price,
                        // Each linear contract value is 1 coin, see:
                        // https://www.bybit.com/data/basic/linear/contract-detail?symbol=BTCUSDT
                        quantity_base: raw_trade.size,
                        quantity_quote: price * raw_trade.size,
                        quantity_contract: Some(raw_trade.size),
                        side: if raw_trade.side == "Sell" {
                            TradeSide::Sell
                        } else {
                            TradeSide::Buy
                        },
                        trade_id: raw_trade.trade_id.clone(),
                        raw: serde_json::to_value(&raw_trade).unwrap(),
                    }
                })
                .collect();

            Ok(trades)
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}

pub(crate) fn parse_l2(_market_type: MarketType, _msg: &str) -> Result<Vec<OrderBookMsg>> {
    Ok(Vec::new())
}
