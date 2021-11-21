use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use crate::exchanges::utils::calc_quantity_and_volume;

const EXCHANGE_NAME: &str = "deribit";

// see https://docs.deribit.com/?javascript#trades-kind-currency-interval
#[derive(Serialize, Deserialize)]
struct RawTradeMsg {
    trade_seq: i64,
    trade_id: String,
    timestamp: i64,
    price: f64,
    instrument_name: String,
    direction: String, // buy, sell
    amount: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.deribit.com/?javascript#book-instrument_name-interval
#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    #[serde(rename = "type")]
    type_: String, // snapshot, change
    timestamp: i64,
    instrument_name: String,
    bids: Vec<[Value; 3]>,
    asks: Vec<[Value; 3]>,
    change_id: Option<u64>,
    prev_change_id: Option<u64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Params<T: Sized> {
    channel: String,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    jsonrpc: String,
    method: String,
    params: Params<T>,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let data = ws_msg.params.data;
    if data.is_object() {
        Some(data["instrument_name"].as_str().unwrap().to_string())
    } else if data.is_array() {
        let arr = data.as_array().unwrap();
        let symbols = arr
            .iter()
            .map(|v| v["instrument_name"].as_str().unwrap())
            .collect::<Vec<&str>>();
        Some(symbols[0].to_string())
    } else {
        panic!("Unknown message format: {}", msg);
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<RawTradeMsg>>>(msg)?;
    let mut trades: Vec<TradeMsg> = ws_msg
        .params
        .data
        .into_iter()
        .map(|raw_trade| {
            let pair =
                crypto_pair::normalize_pair(&raw_trade.instrument_name, EXCHANGE_NAME).unwrap();
            let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &pair,
                raw_trade.price,
                raw_trade.amount,
            );

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_name.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp,
                price: raw_trade.price,
                quantity_base,
                quantity_quote,
                quantity_contract,
                side: if raw_trade.direction == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trade_id.to_string(),
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
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderbookMsg>>(msg)?;
    let raw_orderbook = ws_msg.params.data;
    let snapshot = raw_orderbook.type_ == "snapshot";
    let timestamp = raw_orderbook.timestamp;
    let symbol = raw_orderbook.instrument_name;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    let parse_order = |raw_order: &[Value; 3]| -> Order {
        let price = raw_order[1].as_f64().unwrap();
        let quantity = raw_order[2].as_f64().unwrap();

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
        symbol,
        pair: pair.clone(),
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: raw_orderbook.change_id,
        prev_seq_id: raw_orderbook.prev_change_id,
        asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
        bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
