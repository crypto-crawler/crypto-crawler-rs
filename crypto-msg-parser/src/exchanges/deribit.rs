use crypto_market_type::MarketType;

use crate::{order::Order, MessageType, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

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

fn calc_quantity_and_volume(market_type: MarketType, price: f64, amount: f64) -> (f64, f64) {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            // amount, Trade amount. For perpetual and futures - in USD units
            // see https://docs.deribit.com/?javascript#trades-instrument_name-interval
            let volume = amount;
            (volume / price, volume)
        }
        MarketType::EuropeanOption => (amount, amount * price),
        _ => panic!("Unknown market_type {}", market_type),
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<RawTradeMsg>>>(msg)?;
    let trades: Vec<TradeMsg> = ws_msg
        .params
        .data
        .into_iter()
        .map(|raw_trade| {
            let (quantity_base, quantity_quote) =
                calc_quantity_and_volume(market_type, raw_trade.price, raw_trade.amount);

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_name.clone(),
                pair: crypto_pair::normalize_pair(&raw_trade.instrument_name, EXCHANGE_NAME)
                    .unwrap(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp,
                price: raw_trade.price,
                quantity_base,
                quantity_quote,
                quantity_contract: Some(raw_trade.amount),
                side: if raw_trade.direction == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trade_id.to_string(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

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

        let (quantity_base, quantity_quote) =
            calc_quantity_and_volume(market_type, price, quantity);

        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract: Some(quantity),
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
        bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        raw: serde_json::from_str(msg)?,
    };

    Ok(vec![orderbook])
}
