use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::messages::{WebsocketCandlest, WebsocketMsg};
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide, KlineMsg};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "gate";

// https://www.gateio.pro/docs/apiv4/ws/en/#server-notification-2
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    id: i64,
    create_time: f64,
    create_time_ms: String,
    side: String, // buy, sell
    currency_pair: String,
    amount: String,
    price: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gateio.pro/docs/apiv4/ws/en/#changed-order-book-levels
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookUpdateMsg {
    t: i64,
    e: String,
    E: i64,
    s: String,
    U: i64,
    u: i64,
    a: Option<Vec<[String; 2]>>,
    b: Option<Vec<[String; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gateio.pro/docs/apiv4/ws/en/#limited-level-full-order-book-snapshot
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookSnapshotMsg {
    t: i64,
    lastUpdateId: i64,
    s: String,
    asks: Option<Vec<[String; 2]>>,
    bids: Option<Vec<[String; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/developers/apiv4/en/#retrieve-order-book
#[derive(Serialize, Deserialize)]
struct SpotRestL2SnapshotMsg {
    current: i64,
    update: i64,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<HashMap<String, Value>>>(msg) {
        if let Some(symbol) = ws_msg.result.get("currency_pair") {
            Ok(symbol.as_str().unwrap().to_string())
        } else if let Some(symbol) = ws_msg.result.get("s") {
            Ok(symbol.as_str().unwrap().to_string())
        } else if let Some(symbol) = ws_msg.result.get("n") {
            let n = symbol.as_str().unwrap();
            let pos = n.find('_').unwrap();
            let symbol = &n[(pos + 1)..];
            Ok(symbol.to_string())
        } else {
            Err(SimpleError::new(format!(
                "Unsupported websocket message format {}",
                msg
            )))
        }
    } else if serde_json::from_str::<SpotRestL2SnapshotMsg>(msg).is_ok() {
        Ok("NONE".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<HashMap<String, Value>>>(msg) {
        if ws_msg.channel == "spot.trades" {
            Ok(Some(
                ws_msg.result["create_time_ms"]
                    .as_str()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap() as i64,
            ))
        } else if ws_msg.channel.starts_with("spot.order_book")
            || ws_msg.channel == "spot.book_ticker"
        {
            Ok(Some(ws_msg.result["t"].as_i64().unwrap()))
        } else {
            Ok(Some(ws_msg.time * 1000))
        }
    } else if let Ok(l2_snapshot) = serde_json::from_str::<SpotRestL2SnapshotMsg>(msg) {
        Ok(Some(l2_snapshot.current))
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotTradeMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.channel, "spot.trades");
    debug_assert_eq!(ws_msg.event, "update");
    let result = ws_msg.result;
    let symbol = result.currency_pair;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let price = result.price.parse::<f64>().unwrap();
    let quantity_base = result.amount.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::Trade,
        timestamp: result.create_time_ms.parse::<f64>().unwrap() as i64,
        price: result.price.parse::<f64>().unwrap(),
        quantity_base,
        quantity_quote: price * quantity_base,
        quantity_contract: None,
        side: if result.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: result.id.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

fn parse_order(raw_order: &[String; 2]) -> Order {
    let price = raw_order[0].parse::<f64>().unwrap();
    let quantity_base = raw_order[1].parse::<f64>().unwrap();
    Order {
        price,
        quantity_base,
        quantity_quote: price * quantity_base,
        quantity_contract: None,
    }
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    if ws_msg.channel == "spot.order_book_update" {
        parse_l2_update(msg)
    } else if ws_msg.channel == "spot.order_book" {
        parse_l2_snapshot(msg)
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}

fn parse_l2_update(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<SpotOrderbookUpdateMsg>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<SpotOrderbookUpdateMsg>",
                msg
            ))
        })?;
    debug_assert_eq!(ws_msg.channel, "spot.order_book_update");
    let result = ws_msg.result;
    let symbol = result.s;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp: result.t,
        seq_id: Some(result.u as u64),
        prev_seq_id: Some(result.U as u64 - 1),
        asks: if let Some(asks) = result.a {
            asks.iter().map(parse_order).collect()
        } else {
            Vec::new()
        },
        bids: if let Some(bids) = result.b {
            bids.iter().map(parse_order).collect()
        } else {
            Vec::new()
        },
        snapshot: ws_msg.event == "all",
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

fn parse_l2_snapshot(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<SpotOrderbookSnapshotMsg>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<SpotOrderbookSnapshotMsg>",
                msg
            ))
        })?;
    debug_assert_eq!(ws_msg.channel, "spot.order_book");
    debug_assert_eq!(ws_msg.event, "update");
    let result = ws_msg.result;
    let symbol = result.s;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let parse_order = |raw_order: &[String; 2]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();
        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp: result.t,
        seq_id: None,
        prev_seq_id: None,
        asks: if let Some(asks) = result.asks {
            asks.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        bids: if let Some(bids) = result.bids {
            bids.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        snapshot: true,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

// https://www.gateio.pro/docs/developers/apiv4/zh_CN/#%E5%90%88%E7%BA%A6%E5%B8%82%E5%9C%BA-k-%E7%BA%BF%E5%9B%BE-3
// {
// 	"time": 1654080052,
// 	"channel": "spot.candlesticks",
// 	"event": "update",
// 	"result": {
// 		"t": "1654080050",
// 		"v": "0",
// 		"c": "31555.75",
// 		"h": "31555.75",
// 		"l": "31555.75",
// 		"o": "31555.75",
// 		"n": "10s_BTC_USDT",
// 		"a": "0"
// 	}
// }

#[derive(Serialize, Deserialize)]
struct RawKlineMsg {
    t: String,          // 秒 s 精度的 Unix 时间戳
    v: String,          // integer交易量，只有市场行情的 K 线数据里有该值
    c: String,          // 收盘价
    h: String,          // 最高价
    l: String,          // 最低价
    o: String,          // 开盘价
    n: String,
    a: String,          //基础货币交易量
}


pub(super) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
    msg_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    let obj = serde_json::from_str::<WebsocketCandlest<RawKlineMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketCandlest<RawKlineMsg>",
            msg
        ))
    })?;

    let ch: Vec<&str>= obj.result.n.split("_").collect();
    let symbol = [ch[1].to_string(), ch[2].to_string()].join("_");
    //10s 1m, 5m, 15m, 30m, 1h, 4h, 8h, 1d, 7d, 30d
    let period = ch[0].to_string();

    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    let kline_msg = KlineMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type,
        timestamp: obj.result.t.parse::<i64>().unwrap(),
        json: msg.to_string(),
        open: obj.result.o.parse::<f64>().unwrap(),
        high: obj.result.h.parse::<f64>().unwrap(),
        low: obj.result.l.parse::<f64>().unwrap(),
        close: obj.result.c.parse::<f64>().unwrap(),
        volume: obj.result.v.parse::<f64>().unwrap(),
        period,
        quote_volume: None
    };

    Ok(kline_msg)
}
