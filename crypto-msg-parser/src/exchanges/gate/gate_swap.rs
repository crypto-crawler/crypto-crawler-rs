use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::{super::utils::calc_quantity_and_volume, messages::WebsocketMsg};

use crate::{BboMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::{cell::RefCell, collections::HashMap};
use super::EXCHANGE_NAME;

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
    asks: Vec<RawOrderLegacy>,
    bids: Vec<RawOrderLegacy>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/delivery/ws/index.html#order_book-api
// https://www.gate.io/docs/futures/ws/index.html#legacy-order-book-notification
#[derive(Serialize, Deserialize)]
struct RawOrderLegacy {
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
struct SwapRestL2SnapshotOrder {
    p: String, // price
    s: f64,    // size
}

// https://www.gate.io/docs/developers/apiv4/en/#futures-order-book
// https://www.gate.io/docs/developers/apiv4/en/#futures-order-book-2
#[derive(Serialize, Deserialize)]
struct SwapRestL2SnapshotMsg {
    current: f64,
    update: f64,
    asks: Vec<SwapRestL2SnapshotOrder>,
    bids: Vec<SwapRestL2SnapshotOrder>,
}

// https://www.gateio.pro/docs/apiv4/ws/en/#server-response
// https://www.gate.io/docs/developers/apiv4/ws/en/#best-bid-or-ask-price
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsg {
    t: Option<i64>,         // Order book update time in milliseconds
    u: u64,                 // Order book update ID
    s: String,              // Currency pair
    b: String,              // best bid price
    B: f64,                 // best bid amount
    a: String,              // best ask price
    A: f64,                 // best ask amount
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn extract_symbol(_market_type_: MarketType, msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let v = if ws_msg.result.is_array() {
            ws_msg.result.as_array().unwrap()[0].as_object().unwrap()
        } else {
            ws_msg.result.as_object().unwrap()
        };
        if let Some(symbol) = v.get("contract") {
            Ok(symbol.as_str().unwrap().to_string())
        } else if v.contains_key("s") && v["s"].is_string() {
            Ok(v["s"].as_str().unwrap().to_string())
        } else if v.contains_key("n") && v["n"].is_string() {
            let n = v["n"].as_str().unwrap();
            let pos = n.find('_').unwrap();
            let symbol = &n[(pos + 1)..];
            Ok(symbol.to_string())
        } else if v.contains_key("c") && v["c"].is_string() {
            Ok(v["c"].as_str().unwrap().to_string())
        } else {
            Err(SimpleError::new(format!(
                "Unsupported websocket message format  {}",
                msg
            )))
        }
    } else if msg.contains("open_interest")
        || serde_json::from_str::<SwapRestL2SnapshotMsg>(msg).is_ok()
    {
        Ok("NONE".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format  {}",
            msg
        )))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let result = ws_msg.result;
        if ws_msg.channel == "futures.trades" {
            let timestamp = result
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_object().unwrap())
                .map(|x| {
                    if x.contains_key("create_time_ms") {
                        x["create_time_ms"].as_i64().unwrap()
                    } else {
                        x["create_time"].as_i64().unwrap() * 1000
                    }
                })
                .max();

            if timestamp.is_none() {
                Err(SimpleError::new(format!("result is empty in {}", msg)))
            } else {
                Ok(timestamp)
            }
        } else if ws_msg.channel == "futures.order_book" {
            if let Some(x) = result.get("t") {
                Ok(Some(x.as_i64().unwrap()))
            } else {
                Ok(Some(ws_msg.time * 1000))
            }
        } else if ws_msg.channel == "futures.order_book_update"
            || ws_msg.channel == "futures.book_ticker"
        {
            Ok(Some(result["t"].as_i64().unwrap()))
        } else {
            Ok(Some(ws_msg.time * 1000))
        }
    } else if let Ok(l2_snapshot) = serde_json::from_str::<SwapRestL2SnapshotMsg>(msg) {
        Ok(Some((l2_snapshot.current * 1000.0) as i64))
    } else if msg.contains("open_interest") {
        Ok(None)
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format  {}",
            msg
        )))
    }
}

pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    match market_type {
        MarketType::InverseFuture | MarketType::LinearFuture => {
            let ws_msg =
                serde_json::from_str::<WebsocketMsg<Vec<FutureTradeMsg>>>(msg).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to WebsocketMsg<Vec<FutureTradeMsg>>",
                        msg
                    ))
                })?;

            let mut trades: Vec<TradeMsg> = ws_msg
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
                        json: serde_json::to_string(&raw_trade).unwrap(),
                    }
                })
                .collect();
            if trades.len() == 1 {
                trades[0].json = msg.to_string();
            }
            Ok(trades)
        }
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_msg =
                serde_json::from_str::<WebsocketMsg<Vec<SwapTradeMsg>>>(msg).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to WebsocketMsg<Vec<SwapTradeMsg>>",
                        msg
                    ))
                })?;

            let mut trades: Vec<TradeMsg> = ws_msg
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
                        json: serde_json::to_string(&raw_trade).unwrap(),
                    }
                })
                .collect();
            if trades.len() == 1 {
                trades[0].json = msg.to_string();
            }
            Ok(trades)
        }
        _ => Err(SimpleError::new(format!(
            "Unknown gate market type {}",
            market_type
        ))),
    }
}

thread_local! {
    // symbol -> price -> (true, ask; false, bid)
    static PRICE_HASHMAP: RefCell<HashMap<String,HashMap<String, bool>>> = RefCell::new(HashMap::new());
}

fn parse_l2_legacy(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.channel, "futures.order_book");
    let snapshot = ws_msg.event == "all";

    let orderbook = if snapshot {
        let raw_orderbook = serde_json::from_value::<RawOrderbookSnapshot>(ws_msg.result.clone())
            .map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to RawOrderbookSnapshot",
                ws_msg.result
            ))
        })?;
        let symbol = raw_orderbook.contract;
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg))
        })?;
        let timestamp = if market_type != MarketType::LinearFuture {
            raw_orderbook.t.unwrap()
        } else {
            ws_msg.time * 1000
        };

        let parse_order = |raw_order: &RawOrderLegacy| -> Order {
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
            seq_id: None,
            prev_seq_id: None,
            snapshot,
            json: msg.to_string(),
        }
    } else {
        let raw_orderbook = serde_json::from_value::<Vec<RawOrderLegacy>>(ws_msg.result.clone())
            .map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to Vec<RawOrderLegacy>",
                    ws_msg.result
                ))
            })?;
        let symbol = if market_type == MarketType::LinearFuture {
            raw_orderbook[0].c.clone().unwrap()
        } else {
            raw_orderbook[0].contract.clone().unwrap()
        };
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg))
        })?;
        let timestamp = ws_msg.time * 1000;

        let parse_order = |raw_order: &RawOrderLegacy| -> Order {
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

        PRICE_HASHMAP.with(|slf| {
            let mut tmp = slf.borrow_mut();
            if !tmp.contains_key(&symbol) {
                tmp.insert(symbol.clone(), HashMap::new());
            }
            let price_map = tmp.get_mut(&symbol).unwrap();

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
                seq_id: None,
                prev_seq_id: None,
                asks,
                bids,
                snapshot,
                json: msg.to_string(),
            }
        })
    };

    Ok(vec![orderbook])
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawOrderNew {
    pub p: String,
    pub s: f64,
}

// https://www.gate.io/docs/futures/ws/en/#order-book-update-notification
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OrderbookUpdateMsg {
    pub t: i64,
    pub s: String,
    pub a: Vec<RawOrderNew>,
    pub b: Vec<RawOrderNew>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

fn parse_order(market_type: MarketType, raw_order: &RawOrderNew, pair: &str) -> Order {
    let price = raw_order.p.parse::<f64>().unwrap();
    let quantity = raw_order.s;

    let (quantity_base, quantity_quote, quantity_contract) =
        calc_quantity_and_volume(EXCHANGE_NAME, market_type, pair, price, quantity);
    Order {
        price,
        quantity_base,
        quantity_quote,
        quantity_contract,
    }
}

fn parse_l2_update(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<OrderbookUpdateMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<OrderbookUpdateMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.channel, "futures.order_book_update");
    let result = ws_msg.result;
    let symbol = result.s;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair: pair.clone(),
        msg_type: MessageType::L2Event,
        timestamp: result.t,
        seq_id: result.extra.get("u").and_then(|v| v.as_u64()),
        prev_seq_id: result
            .extra
            .get("U")
            .and_then(|v| v.as_u64().map(|v| v - 1)),
        asks: result
            .a
            .iter()
            .map(|x| parse_order(market_type, x, &pair))
            .collect(),
        bids: result
            .b
            .iter()
            .map(|x| parse_order(market_type, x, &pair))
            .collect(),
        snapshot: ws_msg.event == "all",
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    if ws_msg.channel == "futures.order_book" {
        parse_l2_legacy(market_type, msg)
    } else if ws_msg.channel == "futures.order_book_update" {
        parse_l2_update(market_type, msg)
    } else {
        Err(SimpleError::new(format!(
            "Unknown channel {} of gate {}",
            ws_msg.channel, market_type
        )))
    }
}

pub(super) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawBboMsg>",
            msg
        ))
    })?;
    debug_assert!(ws_msg.channel.ends_with("book_ticker"));

    let timestamp = if market_type == MarketType::Spot {
        received_at.unwrap()
    } else {
        ws_msg.result.t.unwrap()
    };
    let symbol = ws_msg.result.s.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.result.a.parse::<f64>().unwrap(),
        ws_msg.result.A,
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.result.b.parse::<f64>().unwrap(),
        ws_msg.result.B,
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price: ws_msg.result.a.parse::<f64>().unwrap(),
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.result.b.parse::<f64>().unwrap(),
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: Some(ws_msg.result.u),
        json: msg.to_string(),
    };
    Ok(bbo_msg)
}
