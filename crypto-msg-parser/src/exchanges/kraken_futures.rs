use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kraken_futures";

// https://docs.kraken.com/websockets/#message-trade
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    #[serde(rename = "type")]
    type_: String,
    trade_id: i64,
    sequence: i64,
    maker_order_id: String,
    taker_order_id: String,
    time: String,
    product_id: String,
    size: String,
    price: String,
    side: String, // b, s
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct OrderData {
    price: f64,
    qty: f64,
}

// https://support.kraken.com/hc/en-us/articles/360022635912-Book
#[derive(Serialize, Deserialize)]
struct OrderbookSnapshot {
    feed: String,
    product_id: String,
    timestamp: i64,
    seq: u64,
    bids: Vec<OrderData>,
    asks: Vec<OrderData>,
}

// https://docs.kraken.com/websockets/#message-book
#[derive(Serialize, Deserialize)]
struct OrderbookUpdate {
    feed: String,
    product_id: String,
    side: String,
    seq: u64,
    price: f64,
    qty: f64,
    timestamp: i64,
}

// Book Snapshot:
// ```json5
// {
//     "feed": "book_snapshot",
//     "product_id": "PI_XBTUSD",
//     "timestamp": 1565342712774,
//     "seq": 30007298,
//     "bids": [
//         {
//             "price": 11735.0,
//             "qty": 50000.0
//         },
//         ...
//     ],
//     "asks": [
//         {
//             "price": 11739.0,
//             "qty": 47410.0
//         },
//         ...
//     ],
//     "tickSize": null
// }
// ```
//
// Book:
// ```json5
// {
//     "feed": "book",
//     "product_id": "PI_XBTUSD",
//     "side": "sell",
//     "seq": 30007489,
//     "price": 11741.5,
//     "qty": 10000.0,
//     "timestamp": 1565342713929
// }
// ```
pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    debug_assert_eq!(market_type, MarketType::InverseFuture);
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;

    let snapshot = obj.get("feed").unwrap().eq("book_snapshot");

    let parse_order = |od: &OrderData| -> Order {
        Order {
            price: od.price,
            quantity_base: od.qty,
            //TODO: is that correct?
            quantity_quote: od.price * od.qty,
            quantity_contract: Some(od.qty),
        }
    };

    let orderbooks = if snapshot {
        let orderbook_snapshot = serde_json::from_str::<OrderbookSnapshot>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to OrderbookSnapshot",
                msg
            ))
        })?;

        vec![OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::InverseFuture,
            symbol: orderbook_snapshot.product_id.clone(),
            pair: orderbook_snapshot.product_id,
            msg_type: MessageType::L2Event,
            timestamp: orderbook_snapshot.timestamp,
            seq_id: Some(orderbook_snapshot.seq),
            prev_seq_id: None,
            asks: orderbook_snapshot.asks.iter().map(parse_order).collect(),
            bids: orderbook_snapshot.bids.iter().map(parse_order).collect(),
            snapshot,
            json: msg.to_string(),
        }]
    } else {
        let mut asks: Vec<Order> = Vec::new();
        let mut bids: Vec<Order> = Vec::new();
        let mut process_update = |update: &OrderbookUpdate| {
            if update.side.eq("buy") {
                bids.push(Order {
                    price: update.price,
                    quantity_base: update.qty,
                    quantity_quote: update.price * update.qty,
                    quantity_contract: Some(update.qty),
                })
            }
            if update.side.eq("sell") {
                asks.push(Order {
                    price: update.price,
                    quantity_base: update.qty,
                    quantity_quote: update.price * update.qty,
                    quantity_contract: Some(update.qty),
                })
            }
        };
        let update = serde_json::from_str::<OrderbookUpdate>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to OrderbookUpdate", msg))
        })?;
        process_update(&update);

        vec![OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::InverseFuture,
            symbol: update.product_id.clone(),
            pair: update.product_id,
            msg_type: MessageType::L2Event,
            timestamp: update.timestamp,
            seq_id: Some(update.seq),
            prev_seq_id: None,
            asks,
            bids,
            snapshot,
            json: msg.to_string(),
        }]
    };

    Ok(orderbooks)
}
