use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

use super::message::WebsocketMsg;

const EXCHANGE_NAME: &str = "huobi";

// https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-subscribe-trade-detail-data
// https://huobiapi.github.io/docs/option/v1/en/#subscribe-trade-detail-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct LinearTradeMsg {
    id: i64,
    ts: i64,
    amount: f64,
    quantity: f64,
    trade_turnover: f64,
    price: f64,
    direction: String, // sell, buy
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct TradeTick {
    id: i64,
    ts: i64,
    data: Vec<LinearTradeMsg>,
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<TradeTick>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<TradeTick>",
            msg
        ))
    })?;

    let symbol = ws_msg.ch.split('.').nth(1).unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let mut trades: Vec<TradeMsg> = ws_msg
        .tick
        .data
        .into_iter()
        .map(|raw_trade| TradeMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol: symbol.to_string(),
            pair: pair.to_string(),
            msg_type: MessageType::Trade,
            timestamp: raw_trade.ts,
            price: raw_trade.price,
            quantity_base: raw_trade.quantity,
            quantity_quote: raw_trade.trade_turnover,
            quantity_contract: Some(raw_trade.amount),
            side: if raw_trade.direction == "sell" {
                TradeSide::Sell
            } else {
                TradeSide::Buy
            },
            trade_id: raw_trade.id.to_string(),
            json: serde_json::to_string(&raw_trade).unwrap(),
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}
