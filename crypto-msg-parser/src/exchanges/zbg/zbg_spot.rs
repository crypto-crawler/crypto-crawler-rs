use crypto_market_type::MarketType;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

const EXCHANGE_NAME: &str = "zbg";

// https://zbgapi.github.io/docs/spot/v1/en/#market-trade
// [T, symbol-id, symbol, timestamp, ask/bid, price, quantity]
pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let arr = if msg.starts_with(r#"[["T","#) {
        serde_json::from_str::<Vec<Vec<String>>>(msg)?
    } else if msg.starts_with(r#"["T","#) {
        let tmp = serde_json::from_str::<Vec<String>>(msg)?;
        vec![tmp]
    } else {
        panic!("Invalid trade msg {}", msg);
    };

    let mut trades: Vec<TradeMsg> = arr
        .into_iter()
        .map(|raw_trade| {
            assert_eq!(raw_trade[0], "T");
            let timestamp = raw_trade[2].parse::<i64>().unwrap() * 1000;
            let symbol = raw_trade[3].as_str();
            let side = if raw_trade[4] == "ask" {
                TradeSide::Sell
            } else {
                TradeSide::Buy
            };
            let price = raw_trade[5].parse::<f64>().unwrap();
            let quantity = raw_trade[6].parse::<f64>().unwrap();

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol: symbol.to_string(),
                pair: crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: None,
                side,
                trade_id: timestamp.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

#[derive(Serialize, Deserialize)]
struct OrderbookSnapshot {
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
}

// https://zbgapi.github.io/docs/spot/v1/en/#market-depth
// snapshot：
// [AE, symbol-id, symbol, timestamp, asks:[[price, quantity]], bids[[price, quantity]]]
// update:
// [E, symbol-id, timestamp, symbol, ask/bid, price, quantity]
pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>> {
    let snapshot = msg.starts_with(r#"[["AE","#);

    let orderbooks = if snapshot {
        let arr = serde_json::from_str::<Vec<Vec<Value>>>(msg)?;

        let parse_order = |raw_order: &[Value; 2]| -> Order {
            if raw_order[0].is_string() {
                let price = raw_order[0].as_str().unwrap().parse::<f64>().unwrap();
                let quantity_base = raw_order[1].as_str().unwrap().parse::<f64>().unwrap();

                Order {
                    price,
                    quantity_base,
                    quantity_quote: price * quantity_base,
                    quantity_contract: None,
                }
            } else if raw_order[0].is_f64() {
                let price = raw_order[0].as_f64().unwrap();
                let quantity_base = raw_order[1].as_f64().unwrap();

                Order {
                    price,
                    quantity_base,
                    quantity_quote: price * quantity_base,
                    quantity_contract: None,
                }
            } else {
                panic!("Unknown format {}", msg);
            }
        };

        let mut v = arr
            .iter()
            .map(|raw_orderbook| {
                let symbol = raw_orderbook[2].as_str().unwrap();
                let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
                let timestamp = raw_orderbook[3].as_str().unwrap().parse::<i64>().unwrap() * 1000;

                let asks = serde_json::from_value::<Vec<[Value; 2]>>(
                    raw_orderbook[4]
                        .as_object()
                        .unwrap()
                        .get("asks")
                        .unwrap()
                        .clone(),
                )
                .unwrap()
                .iter()
                .map(|x| parse_order(x))
                .collect::<Vec<Order>>();
                let bids = serde_json::from_value::<Vec<[Value; 2]>>(
                    raw_orderbook[5]
                        .as_object()
                        .unwrap()
                        .get("bids")
                        .unwrap()
                        .clone(),
                )
                .unwrap()
                .iter()
                .map(|x| parse_order(x))
                .collect::<Vec<Order>>();

                OrderBookMsg {
                    exchange: EXCHANGE_NAME.to_string(),
                    market_type: MarketType::Spot,
                    symbol: symbol.to_string(),
                    pair,
                    msg_type: MessageType::L2Event,
                    timestamp,
                    asks,
                    bids,
                    snapshot,
                    json: serde_json::to_string(raw_orderbook)
                        .unwrap()
                        .as_str()
                        .to_string(),
                }
            })
            .collect::<Vec<OrderBookMsg>>();

        if v.len() == 1 {
            v[0].json = msg.to_string();
        }
        v
    } else {
        let arr = serde_json::from_str::<Vec<String>>(msg)?;
        let symbol = arr[3].clone();
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
        let timestamp = arr[2].parse::<i64>().unwrap() * 1000;

        let mut asks: Vec<Order> = Vec::new();
        let mut bids: Vec<Order> = Vec::new();

        let order: Order = {
            let price = arr[5].parse::<f64>().unwrap();
            let quantity_base = arr[6].parse::<f64>().unwrap();

            Order {
                price,
                quantity_base,
                quantity_quote: quantity_base * price,
                quantity_contract: None,
            }
        };

        if arr[4] == "BID" {
            bids.push(order);
        } else {
            asks.push(order);
        }

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::Spot,
            symbol,
            pair,
            msg_type: MessageType::L2Event,
            timestamp,
            asks,
            bids,
            snapshot,
            json: msg.to_string(),
        };
        vec![orderbook]
    };
    Ok(orderbooks)
}
