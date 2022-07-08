use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::exchanges::utils::{calc_quantity_and_volume, deserialize_null_default};
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use super::message::WebsocketMsg;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "huobi";

// see https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#subscribe-trade-detail-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct InverseTradeMsg {
    id: i64,
    ts: i64,
    amount: f64,
    quantity: f64,
    price: f64,
    direction: String, // sell, buy
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://huobiapi.github.io/docs/dm/v1/en/#subscribe-market-depth-data
// https://huobiapi.github.io/docs/dm/v1/en/#subscribe-incremental-market-depth-data
// https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#subscribe-market-depth-data
// https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#subscribe-incremental-market-depth-data
// https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-subscribe-market-depth-data
// https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-subscribe-incremental-market-depth-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct InverseOrderbookMsg {
    id: i64,
    ts: i64,
    mrid: u64,
    event: Option<String>, // snapshot, update, None if L2TopK
    ch: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    bids: Vec<[f64; 2]>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    asks: Vec<[f64; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct TradeTick {
    id: i64,
    ts: i64,
    data: Vec<InverseTradeMsg>,
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<TradeTick>>(msg).map_err(SimpleError::from)?;

    let symbol = ws_msg.ch.split('.').nth(1).unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let mut trades: Vec<TradeMsg> = ws_msg
        .tick
        .data
        .into_iter()
        .map(|raw_trade| {
            let (_, quantity_quote, _) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &pair,
                raw_trade.price,
                raw_trade.amount,
            );
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.to_string(),
                pair: pair.to_string(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.ts,
                price: raw_trade.price,
                quantity_base: raw_trade.quantity,
                quantity_quote,
                quantity_contract: Some(raw_trade.amount),
                side: if raw_trade.direction == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.id.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<InverseOrderbookMsg>>(msg)
        .map_err(SimpleError::from)?;
    let symbol = ws_msg.ch.split('.').nth(1).unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let timestamp = ws_msg.ts;

    let msg_type = if ws_msg.ch.ends_with(".high_freq") {
        MessageType::L2Event
    } else if ws_msg.ch.contains(".depth.step") {
        MessageType::L2TopK
    } else {
        panic!("Unsupported channel {}", ws_msg.ch);
    };

    let snapshot = if msg_type == MessageType::L2Event {
        ws_msg.tick.event.unwrap() == "snapshot"
    } else {
        true
    };

    let parse_order = |raw_order: &[f64; 2]| -> Order {
        let price = raw_order[0];
        let quantity = raw_order[1];

        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.to_string(),
        msg_type,
        timestamp,
        seq_id: Some(ws_msg.tick.mrid),
        prev_seq_id: None,
        asks: ws_msg.tick.asks.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.tick.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
