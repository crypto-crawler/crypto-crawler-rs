use crypto_market_type::MarketType;

use crate::{
    exchanges::utils::calc_quantity_and_volume, MessageType, Order, OrderBookMsg, TradeMsg,
    TradeSide,
};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bybit";

// see https://bybit-exchange.github.io/docs/inverse/#t-websockettrade
#[derive(Serialize, Deserialize)]
struct InverseTradeMsg {
    trade_time_ms: i64,
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy
    size: f64,
    price: f64,
    tick_direction: String,
    trade_id: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://bybit-exchange.github.io/docs/linear/#t-websockettrade
#[derive(Serialize, Deserialize)]
struct LinearTradeMsg {
    trade_time_ms: String,
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy
    size: f64,
    price: String,
    tick_direction: String,
    trade_id: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    topic: String,
    data: Vec<T>,
}

// https://bybit-exchange.github.io/docs/inverse/#t-websocketorderbook25
// https://bybit-exchange.github.io/docs/linear/#t-websocketorderbook25
#[derive(Serialize, Deserialize)]
struct RawOrder {
    price: String,
    symbol: String,
    side: String,
    size: Option<f64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct LinearOrderbookSnapshot {
    order_book: Vec<RawOrder>,
}

#[derive(Serialize, Deserialize)]
struct OrderbookDelta {
    delete: Vec<RawOrder>,
    update: Vec<RawOrder>,
    insert: Vec<RawOrder>,
}

#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    topic: String,
    #[serde(rename = "type")]
    type_: String,
    data: Value,
    timestamp_e6: Value, // i64 or String
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<InverseTradeMsg>>(msg)?;

            let mut trades: Vec<TradeMsg> = ws_msg
                .data
                .into_iter()
                .map(|raw_trade| TradeMsg {
                    exchange: EXCHANGE_NAME.to_string(),
                    market_type,
                    symbol: raw_trade.symbol.clone(),
                    pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
                    msg_type: MessageType::Trade,
                    timestamp: raw_trade.trade_time_ms,
                    price: raw_trade.price,
                    quantity_base: raw_trade.size / raw_trade.price,
                    // Each inverse contract value is 1 USD, see:
                    // https://www.bybit.com/data/basic/inverse/contract-detail?symbol=BTCUSD
                    // https://www.bybit.com/data/basic/future-inverse/contract-detail?symbol=BTCUSD0625
                    quantity_quote: raw_trade.size,
                    quantity_contract: Some(raw_trade.size),
                    side: if raw_trade.side == "Sell" {
                        TradeSide::Sell
                    } else {
                        TradeSide::Buy
                    },
                    trade_id: raw_trade.trade_id.clone(),
                    json: serde_json::to_string(&raw_trade).unwrap(),
                })
                .collect();
            if trades.len() == 1 {
                trades[0].json = msg.to_string();
            }
            Ok(trades)
        }
        MarketType::LinearSwap => {
            let ws_msg = serde_json::from_str::<WebsocketMsg<LinearTradeMsg>>(msg)?;

            let mut trades: Vec<TradeMsg> = ws_msg
                .data
                .into_iter()
                .map(|raw_trade| {
                    let price = raw_trade.price.parse::<f64>().unwrap();
                    TradeMsg {
                        exchange: EXCHANGE_NAME.to_string(),
                        market_type,
                        symbol: raw_trade.symbol.clone(),
                        pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME)
                            .unwrap(),
                        msg_type: MessageType::Trade,
                        timestamp: raw_trade.trade_time_ms.parse::<i64>().unwrap(),
                        price,
                        // Each linear contract value is 1 coin, see:
                        // https://www.bybit.com/data/basic/linear/contract-detail?symbol=BTCUSDT
                        quantity_base: raw_trade.size,
                        quantity_quote: price * raw_trade.size,
                        quantity_contract: Some(raw_trade.size),
                        side: if raw_trade.side == "Sell" {
                            TradeSide::Sell
                        } else {
                            TradeSide::Buy
                        },
                        trade_id: raw_trade.trade_id.clone(),
                        json: serde_json::to_string(&raw_trade).unwrap(),
                    }
                })
                .collect();
            if trades.len() == 1 {
                trades[0].json = msg.to_string();
            }
            Ok(trades)
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<RawOrderbookMsg>(msg)?;
    let symbol = ws_msg.topic.strip_prefix("orderBookL2_25.").unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let snapshot = ws_msg.type_ == "snapshot";
    let timestamp = if ws_msg.timestamp_e6.is_i64() {
        ws_msg.timestamp_e6.as_i64().unwrap()
    } else {
        ws_msg
            .timestamp_e6
            .as_str()
            .unwrap()
            .parse::<i64>()
            .unwrap()
    } / 1000;

    let parse_order = |raw_order: &RawOrder| -> Order {
        let price = raw_order.price.parse::<f64>().unwrap();
        let quantity = raw_order.size.unwrap_or(0.0);
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let mut orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.to_string(),
        msg_type: MessageType::L2Event,
        timestamp,
        asks: Vec::new(),
        bids: Vec::new(),
        snapshot,
        json: msg.to_string(),
    };

    let raw_orders = match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            if snapshot {
                serde_json::from_value::<Vec<RawOrder>>(ws_msg.data).unwrap()
            } else {
                let tmp = serde_json::from_value::<OrderbookDelta>(ws_msg.data).unwrap();
                let mut v = Vec::<RawOrder>::new();
                v.extend(tmp.delete);
                v.extend(tmp.update);
                v.extend(tmp.insert);
                v
            }
        }
        MarketType::LinearSwap => {
            if snapshot {
                let tmp = serde_json::from_value::<LinearOrderbookSnapshot>(ws_msg.data).unwrap();
                tmp.order_book
            } else {
                let tmp = serde_json::from_value::<OrderbookDelta>(ws_msg.data).unwrap();
                let mut v = Vec::<RawOrder>::new();
                v.extend(tmp.delete);
                v.extend(tmp.update);
                v.extend(tmp.insert);
                v
            }
        }
        _ => panic!("Unknown market_type {}", market_type),
    };

    for raw_order in raw_orders.iter() {
        let order = parse_order(raw_order);
        if raw_order.side == "Buy" {
            orderbook.bids.push(order);
        } else {
            orderbook.asks.push(order);
        }
    }
    Ok(vec![orderbook])
}
