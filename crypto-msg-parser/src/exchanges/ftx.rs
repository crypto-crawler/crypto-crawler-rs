use crypto_market_type::MarketType;

use crate::{
    exchanges::utils::calc_quantity_and_volume, MessageType, Order, OrderBookMsg, TradeMsg,
    TradeSide,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "ftx";

// https://docs.ftx.com/#trades
#[derive(Serialize, Deserialize)]
struct RawTradeMsg {
    id: i64,
    price: f64,
    size: f64,
    side: String, // buy, sell
    liquidation: bool,
    time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.ftx.com/#orderbooks
#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    action: String, // partial, update
    bids: Vec<[f64; 2]>,
    asks: Vec<[f64; 2]>,
    time: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    market: String,
    #[serde(rename = "type")]
    type_: String,
    data: T,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<RawTradeMsg>>>(msg)?;
    let symbol = ws_msg.market.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.time).unwrap();

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.to_string(),
                pair: pair.clone(),
                msg_type: MessageType::Trade,
                timestamp: timestamp.timestamp_millis(),
                price: raw_trade.price,
                quantity_base: raw_trade.size,
                quantity_quote: raw_trade.price * raw_trade.size,
                quantity_contract: if market_type == MarketType::Spot {
                    None
                } else {
                    Some(raw_trade.size)
                },
                side: if raw_trade.side == "sell" {
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

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderbookMsg>>(msg)?;
    debug_assert_eq!(ws_msg.channel, "orderbook");
    let symbol = ws_msg.market.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let snapshot = ws_msg.data.action == "partial";
    let timestamp = (ws_msg.data.time * 1000.0) as i64;

    let parse_order = |raw_order: &[f64; 2]| -> Order {
        let price = raw_order[0];
        let quantity = raw_order[1];
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

        if let Some(qc) = quantity_contract {
            vec![price, quantity_base, quantity_quote, qc]
        } else {
            vec![price, quantity_base, quantity_quote]
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.to_string(),
        msg_type: MessageType::L2Event,
        timestamp,
        asks: ws_msg.data.asks.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.data.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        raw: serde_json::from_str(msg)?,
    };

    Ok(vec![orderbook])
}
