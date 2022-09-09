use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{CandlestickMsg, TradeMsg, TradeSide};

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

// https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-subscribe-kline-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawCandlestickMsg {
    id: i64,
    mrid: i64,
    open: f64,
    close: f64,
    low: f64,
    high: f64,
    amount: f64,
    vol: f64,
    trade_turnover: f64,
    count: u64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
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

pub(super) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<CandlestickMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<RawCandlestickMsg>>(msg).map_err(SimpleError::from)?;
    debug_assert!(ws_msg.ch.contains(".kline."));

    let (symbol, period) = {
        let arr: Vec<&str> = ws_msg.ch.split('.').collect();
        let symbol = arr[1];
        let period = arr[3];
        (symbol, period)
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let kline_msg = CandlestickMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        msg_type: MessageType::Candlestick,
        symbol: symbol.to_string(),
        pair,
        timestamp: ws_msg.ts,
        begin_time: ws_msg.tick.id,
        open: ws_msg.tick.open,
        high: ws_msg.tick.high,
        low: ws_msg.tick.low,
        close: ws_msg.tick.close,
        volume: ws_msg.tick.amount,
        quote_volume: Some(ws_msg.tick.trade_turnover),
        period: period.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![kline_msg])
}
