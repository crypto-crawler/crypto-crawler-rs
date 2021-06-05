use crypto_market_type::MarketType;

use super::super::utils::calc_quantity_and_volume;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::{collections::HashMap, sync::Mutex};

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

// https://www.gate.io/docs/delivery/ws/index.html#order_book-api
// https://www.gate.io/docs/futures/ws/index.html#legacy-order-book-notification
#[derive(Serialize, Deserialize)]
struct RawOrderbookSnapshot {
    t: Option<i64>,
    contract: String,
    asks: Vec<RawOrder>,
    bids: Vec<RawOrder>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/delivery/ws/index.html#order_book-api
// https://www.gate.io/docs/futures/ws/index.html#legacy-order-book-notification
#[derive(Serialize, Deserialize)]
struct RawOrder {
    p: String, // price
    s: f64,    // size, -, asks; +, bids
    contract: Option<String>,
    c: Option<String>, // LinearFuture
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
    result: T,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::LinearFuture => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<FutureTradeMsg>>>(msg)?;

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
            let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<SwapTradeMsg>>>(msg)?;

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

lazy_static! {
    // symbol -> price -> (true, ask; false, bid)
    static ref PRICE_HASHMAP: Mutex<HashMap<String,HashMap<String, bool>>> = Mutex::new(HashMap::new());
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)?;
    debug_assert_eq!(ws_msg.channel, "futures.order_book");
    let snapshot = ws_msg.event == "all";

    let orderbook = if snapshot {
        let raw_orderbook = serde_json::from_value::<RawOrderbookSnapshot>(ws_msg.result).unwrap();
        let symbol = raw_orderbook.contract;
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
        let timestamp = if market_type != MarketType::LinearFuture {
            raw_orderbook.t.unwrap()
        } else {
            ws_msg.time * 1000
        };

        let parse_order = |raw_order: &RawOrder| -> Order {
            let price = raw_order.p.parse::<f64>().unwrap();
            let quantity = raw_order.s;

            let (quantity_base, quantity_quote, quantity_contract) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
            Order {
                price,
                quantity_base,
                quantity_quote,
                quantity_contract,
            }
        };

        OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            pair: pair.to_string(),
            msg_type: MessageType::L2Event,
            timestamp,
            asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
            bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
            snapshot,
            raw: serde_json::from_str(msg)?,
        }
    } else {
        let raw_orderbook = serde_json::from_value::<Vec<RawOrder>>(ws_msg.result).unwrap();
        let symbol = if market_type == MarketType::LinearFuture {
            raw_orderbook[0].c.clone().unwrap()
        } else {
            raw_orderbook[0].contract.clone().unwrap()
        };
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
        let timestamp = ws_msg.time * 1000;

        let parse_order = |raw_order: &RawOrder| -> Order {
            let price = raw_order.p.parse::<f64>().unwrap();
            let quantity = f64::abs(raw_order.s);

            let (quantity_base, quantity_quote, quantity_contract) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
            Order {
                price,
                quantity_base,
                quantity_quote,
                quantity_contract,
            }
        };

        let mut guard = PRICE_HASHMAP.lock().unwrap();
        if !guard.contains_key(&symbol) {
            guard.insert(symbol.clone(), HashMap::new());
        }
        let price_map = guard.get_mut(&symbol).unwrap();

        let mut asks: Vec<Order> = Vec::new();
        let mut bids: Vec<Order> = Vec::new();
        for x in raw_orderbook.iter() {
            let price = x.p.clone();
            let order = parse_order(x);
            if x.s < 0.0 {
                asks.push(order);
                price_map.insert(price, true);
            } else if x.s > 0.0 {
                bids.push(order);
                price_map.insert(price, false);
            } else if let Some(ask) = price_map.remove(&price) {
                if ask {
                    asks.push(order);
                } else {
                    bids.push(order);
                }
            }
        }

        OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            pair: pair.to_string(),
            msg_type: MessageType::L2Event,
            timestamp,
            asks,
            bids,
            snapshot,
            raw: serde_json::from_str(msg)?,
        }
    };

    Ok(vec![orderbook])
}
