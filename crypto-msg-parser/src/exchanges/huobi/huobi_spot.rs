use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{BboMsg, CandlestickMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

use super::message::WebsocketMsg;

const EXCHANGE_NAME: &str = "huobi";

// see https://huobiapi.github.io/docs/spot/v1/en/#trade-detail
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotTradeMsg {
    ts: i64,
    tradeId: i64,
    amount: f64,
    price: f64,
    direction: String, // sell, buy
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://huobiapi.github.io/docs/spot/v1/en/#market-by-price-incremental-update
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookMsg {
    #[serde(rename = "seqNum")]
    seq_num: Option<u64>, // None if L2TopK
    #[serde(rename = "prevSeqNum")]
    prev_seq_num: Option<u64>, // None if L2TopK
    asks: Option<Vec<[f64; 2]>>,
    bids: Option<Vec<[f64; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://huobiapi.github.io/docs/spot/v1/en/#best-bid-offer
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsg {
    seqId: Option<u64>,
    ask: f64,
    askSize: f64,
    bid: f64,
    bidSize: f64,
    quoteTime: i64,
    symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct TradeTick {
    id: i64,
    ts: i64,
    data: Vec<SpotTradeMsg>,
}

// https://huobiapi.github.io/docs/spot/v1/en/#market-candlestick
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawCandlestickMsg {
    id: i64,
    open: f64,
    close: f64,
    low: f64,
    high: f64,
    amount: f64,
    vol: f64,
    count: u64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
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
            market_type: MarketType::Spot,
            symbol: symbol.to_string(),
            pair: pair.to_string(),
            msg_type: MessageType::Trade,
            timestamp: raw_trade.ts,
            price: raw_trade.price,
            quantity_base: raw_trade.amount,
            quantity_quote: raw_trade.price * raw_trade.amount,
            quantity_contract: None,
            side: if raw_trade.direction == "sell" {
                TradeSide::Sell
            } else {
                TradeSide::Buy
            },
            trade_id: raw_trade.tradeId.to_string(),
            json: serde_json::to_string(&raw_trade).unwrap(),
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotOrderbookMsg>",
            msg
        ))
    })?;
    let symbol = ws_msg.ch.split('.').nth(1).unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let timestamp = ws_msg.ts;

    let parse_order = |raw_order: &[f64; 2]| -> Order {
        let price = raw_order[0];
        let quantity_base = raw_order[1];

        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };
    let msg_type = if ws_msg.ch.contains(".mbp.") {
        MessageType::L2Event
    } else if ws_msg.ch.contains(".depth.step") {
        MessageType::L2TopK
    } else {
        panic!("Unsupported channel {}", ws_msg.ch);
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type,
        timestamp,
        seq_id: ws_msg.tick.seq_num,
        prev_seq_id: ws_msg.tick.prev_seq_num,
        asks: ws_msg
            .tick
            .asks
            .into_iter()
            .flatten()
            .map(|x| parse_order(&x))
            .collect(),
        bids: ws_msg
            .tick
            .bids
            .into_iter()
            .flatten()
            .map(|x| parse_order(&x))
            .collect(),
        snapshot: msg_type == MessageType::L2TopK,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

pub(super) fn parse_bbo(msg: &str) -> Result<Vec<BboMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsg>>(msg).map_err(SimpleError::from)?;
    debug_assert!(ws_msg.ch.ends_with(".bbo"));

    let symbol = ws_msg.ch.split('.').nth(1).unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp: ws_msg.ts,
        ask_price: ws_msg.tick.ask,
        ask_quantity_base: ws_msg.tick.askSize,
        ask_quantity_quote: ws_msg.tick.ask * ws_msg.tick.askSize,
        ask_quantity_contract: None,
        bid_price: ws_msg.tick.bid,
        bid_quantity_base: ws_msg.tick.bidSize,
        bid_quantity_quote: ws_msg.tick.bid * ws_msg.tick.bidSize,
        bid_quantity_contract: None,
        id: ws_msg.tick.seqId,
        json: msg.to_string(),
    };

    Ok(vec![bbo_msg])
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
        quote_volume: Some(ws_msg.tick.vol),
        period: period.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![kline_msg])
}
