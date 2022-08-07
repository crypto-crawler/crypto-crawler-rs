use crypto_market_type::MarketType;
use crypto_message::{BboMsg, CandlestickMsg, Order, OrderBookMsg, TradeMsg, TradeSide};
use crypto_msg_type::MessageType;

use super::super::utils::calc_quantity_and_volume;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

use super::message::WebsocketMsg;

const EXCHANGE_NAME: &str = "kucoin";

// https://docs.kucoin.com/#match-execution-data
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    symbol: String,
    sequence: String,
    side: String, // buy, sell
    size: String,
    price: String,
    time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Changes {
    asks: Vec<[String; 3]>, //price, size, sequence
    bids: Vec<[String; 3]>,
}

// https://docs.kucoin.com/#level-2-market-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookMsg {
    sequenceStart: i64,
    symbol: String,
    changes: Changes,
    sequenceEnd: i64,
    time: Option<i64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.kucoin.com/#level2-5-best-ask-bid-orders
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotL2TopKMsg {
    timestamp: i64,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See <https://docs.kucoin.com/#symbol-ticker>
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsg {
    sequence: String,
    price: String,
    size: String,
    bestAsk: String,
    bestAskSize: String,
    bestBid: String,
    bestBidSize: String,
    time: i64,
}

// See <https://docs.kucoin.com/#klines>
#[derive(Serialize, Deserialize)]
struct RawCandlestickMsg {
    symbol: String,
    time: i64,
    candles: Vec<String>,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotOrderbookMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.subject, "trade.l3match");
    debug_assert!(ws_msg.topic.starts_with("/market/match:"));
    let raw_trade = ws_msg.data;
    let price = raw_trade.price.parse::<f64>().unwrap();
    let quantity = raw_trade.size.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: raw_trade.symbol.clone(),
        pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!(
                "Failed to normalize {} from {}",
                raw_trade.symbol, msg
            ))
        })?,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.time.parse::<i64>().unwrap() / 1000000,
        price,
        quantity_base: quantity,
        quantity_quote: price * quantity,
        quantity_contract: None,
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.sequence.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

pub(super) fn parse_l2(msg: &str, timestamp: i64) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotOrderbookMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.subject, "trade.l2update");
    debug_assert!(ws_msg.topic.starts_with("/market/level2:"));
    let symbol = ws_msg.data.symbol;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let parse_order = |raw_order: &[String; 3]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();

        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp: ws_msg.data.time.unwrap_or(timestamp),
        seq_id: Some(ws_msg.data.sequenceStart as u64),
        prev_seq_id: None,
        asks: ws_msg
            .data
            .changes
            .asks
            .iter()
            .map(|x| parse_order(x))
            .collect(),
        bids: ws_msg
            .data
            .changes
            .bids
            .iter()
            .map(|x| parse_order(x))
            .collect(),
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

pub(super) fn parse_l2_topk(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotL2TopKMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotOrderbookMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.subject, "level2");
    debug_assert!(ws_msg.topic.starts_with("/spotMarket/level2Depth5:"));
    let symbol = ws_msg.topic.split(':').last().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let timestamp = ws_msg.data.timestamp;

    let parse_order = |raw_order: &[String; 2]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();

        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2TopK,
        timestamp,
        seq_id: None,
        prev_seq_id: None,
        asks: ws_msg.data.asks.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.data.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot: true,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

pub(super) fn parse_bbo(msg: &str) -> Result<Vec<BboMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsg>>(msg).map_err(SimpleError::from)?;
    let topic = ws_msg.topic.as_str();

    let symbol = if topic == "/market/ticker:all" {
        ws_msg.subject.as_str()
    } else {
        topic.split(':').last().unwrap()
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        MarketType::Spot,
        &pair,
        ws_msg.data.bestAsk.parse::<f64>().unwrap(),
        ws_msg.data.bestAskSize.parse::<f64>().unwrap(),
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        MarketType::Spot,
        &pair,
        ws_msg.data.bestBid.parse::<f64>().unwrap(),
        ws_msg.data.bestBidSize.parse::<f64>().unwrap(),
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp: ws_msg.data.time,
        ask_price: ws_msg.data.bestAsk.parse::<f64>().unwrap(),
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.data.bestBid.parse::<f64>().unwrap(),
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: Some(ws_msg.data.sequence.as_str().parse::<u64>().unwrap()),
        json: msg.to_string(),
    };
    Ok(vec![bbo_msg])
}

pub(super) fn parse_candlestick(msg: &str) -> Result<Vec<CandlestickMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<RawCandlestickMsg>>(msg).map_err(SimpleError::from)?;

    let symbol = ws_msg.data.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let begin_time = ws_msg.data.candles[0].parse::<i64>().unwrap();
    let open: f64 = ws_msg.data.candles[1].parse().unwrap();
    let close: f64 = ws_msg.data.candles[2].parse().unwrap();
    let high: f64 = ws_msg.data.candles[3].parse().unwrap();
    let low: f64 = ws_msg.data.candles[4].parse().unwrap();
    let volume: f64 = ws_msg.data.candles[5].parse().unwrap();
    let quote_volume: f64 = ws_msg.data.candles[6].parse().unwrap();

    let period = ws_msg
        .topic
        .split(':')
        .last()
        .unwrap()
        .split('_')
        .last()
        .unwrap()
        .to_string();

    let kline_msg = CandlestickMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: ws_msg.data.symbol.to_owned(),
        pair,
        msg_type: MessageType::Candlestick,
        timestamp: ws_msg.data.time / 1000000,

        begin_time,
        open,
        high,
        low,
        close,
        volume,
        quote_volume: Some(quote_volume),
        period,
        json: msg.to_string(),
    };

    Ok(vec![kline_msg])
}
