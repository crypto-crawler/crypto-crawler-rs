use crypto_market_type::MarketType;

use crate::{
    exchanges::utils::calc_quantity_and_volume, MessageType, Order, OrderBookMsg, TradeMsg,
    TradeSide,
};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use super::message::WebsocketMsg;

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

// https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#subscribe-incremental-market-depth-data
// https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-subscribe-incremental-market-depth-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct InverseOrderbookMsg {
    id: i64,
    ts: i64,
    event: String, // snapshot, update
    ch: String,
    bids: Vec<[f64; 2]>,
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

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<TradeTick>>(msg)?;

    let symbol = {
        let v: Vec<&str> = ws_msg.ch.split('.').collect();
        v[1]
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

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

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<InverseOrderbookMsg>>(msg)?;
    let symbol = {
        let v: Vec<&str> = ws_msg.tick.ch.split('.').collect();
        v[1]
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let timestamp = ws_msg.tick.ts;
    let snapshot = ws_msg.tick.event == "snapshot";

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
        msg_type: MessageType::L2Event,
        timestamp,
        seq_first: Some(ws_msg.tick.id as u64),
        seq_last: None,
        asks: ws_msg.tick.asks.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.tick.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
