use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::exchanges::utils::calc_quantity_and_volume;
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide, CandlestickMsg};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
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

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if json_obj.contains_key("topic") && json_obj["topic"].is_string() {
        let symbol = json_obj["topic"]
            .as_str()
            .unwrap()
            .split('.')
            .last()
            .unwrap();
        Ok(symbol.to_string())
    } else if json_obj.contains_key("ret_code")
        && json_obj.contains_key("ret_msg")
        && json_obj.contains_key("result")
    {
        // Data from RESTful APIs
        if json_obj["ret_code"].as_i64().unwrap() != 0 {
            return Err(SimpleError::new(format!("Error HTTP response {}", msg)));
        }
        let arr = json_obj["result"].as_array().unwrap();
        Ok(arr[0]["symbol"].as_str().unwrap().to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if json_obj.contains_key("topic") && json_obj["topic"].is_string() {
        let msg_type = json_obj["topic"]
            .as_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();
        match msg_type {
            "trade" => {
                let raw_trades = json_obj["data"].as_array().unwrap();
                let timestamp = raw_trades
                    .iter()
                    .map(|raw_trade| {
                        if raw_trade["trade_time_ms"].is_i64() {
                            raw_trade["trade_time_ms"].as_i64().unwrap()
                        } else {
                            raw_trade["trade_time_ms"]
                                .as_str()
                                .unwrap()
                                .parse::<i64>()
                                .unwrap()
                        }
                    })
                    .max();

                if timestamp.is_none() {
                    Err(SimpleError::new(format!("data is empty in {}", msg)))
                } else {
                    Ok(timestamp)
                }
            }
            _ => {
                let timestamp_e6 = &json_obj["timestamp_e6"];
                let timestamp = if timestamp_e6.is_i64() {
                    timestamp_e6.as_i64().unwrap()
                } else {
                    timestamp_e6.as_str().unwrap().parse::<i64>().unwrap()
                } / 1000;
                Ok(Some(timestamp))
            }
        }
    } else if json_obj.contains_key("ret_code")
        && json_obj.contains_key("ret_msg")
        && json_obj.contains_key("result")
    {
        // Data from RESTful APIs
        if json_obj["ret_code"].as_i64().unwrap() != 0 {
            return Err(SimpleError::new(format!("Error HTTP response {}", msg)));
        }
        Ok(json_obj
            .get("time_now")
            .map(|x| (x.as_str().unwrap().parse::<f64>().unwrap() * 1000.0) as i64))
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract timestamp from {}",
            msg
        )))
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<HashMap<String, Value>>(msg) {
        let table = ws_msg.get("topic").unwrap().as_str().unwrap();
        let channel = {
            let arr = table.split('.').collect::<Vec<&str>>();
            arr[0]
        };
        if channel == "trade" {
            MessageType::Trade
        } else if channel == "orderBookL2_25" {
            MessageType::L2Event
        } else if table == "instrument_info" {
            MessageType::Ticker
        } else if table == "klineV2" || table == "candle" {
            MessageType::Candlestick
        } else {
            MessageType::Other
        }
    } else {
        MessageType::Other
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            let ws_msg =
                serde_json::from_str::<WebsocketMsg<InverseTradeMsg>>(msg).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to WebsocketMsg<InverseTradeMsg>",
                        msg
                    ))
                })?;

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
            let ws_msg =
                serde_json::from_str::<WebsocketMsg<LinearTradeMsg>>(msg).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to WebsocketMsg<LinearTradeMsg>",
                        msg
                    ))
                })?;

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
        _ => Err(SimpleError::new(format!(
            "Unknown market_type {}",
            market_type
        ))),
    }
}


// See: https://www.bybit.com/data/basic/inverse/contract-detail?symbol=BTCUSD
// Sample:
// {"topic":"klineV2.5.BTCUSD","data":[{"start":1660414200,"end":1660414500,"open":24555.5,"close":24554,"high":24555.5,"low":24548,"volume":61397,"turnover":2.50100659,"confirm":false,"cross_seq":14996121952,"timestamp":1660414346404174}],"timestamp_e6":1660414346404174}

// Parse candlestick data from websocket message
pub(crate) fn parse_candlestick(market_type: MarketType, msg: &str) -> Result<Vec<CandlestickMsg>, SimpleError> {

    // Get JSOn object from message
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    // Convert JSON object to CandlestickMsg
    // panic!("{}", msg);
    let candlestick_msg = CandlestickMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        // Get symbol from topic
        symbol: json_obj
            .get("topic")
            .unwrap()
            .as_str()
            .unwrap()
            .split('.')
            .nth(1)
            .unwrap()
            .to_string(),
        // symbol: json_obj.get("symbol").unwrap().as_str().unwrap().to_string(),
        pair: crypto_pair::normalize_pair(
            json_obj.get("symbol").unwrap().as_str().unwrap(),
            EXCHANGE_NAME,
        )
        .unwrap(),
        msg_type: MessageType::Candlestick,
        timestamp: json_obj.get("time_now").unwrap().as_str().unwrap().parse::<i64>().unwrap(),
        open: json_obj.get("open").unwrap().as_f64().unwrap(),
        high: json_obj.get("high").unwrap().as_f64().unwrap(),
        low: json_obj.get("low").unwrap().as_f64().unwrap(),
        close: json_obj.get("close").unwrap().as_f64().unwrap(),
        volume: json_obj.get("volume").unwrap().as_f64().unwrap(),
        begin_time: json_obj.get("start").unwrap().as_i64().unwrap(),
        // Get period from start and end time
        period: parse_candlestick_period(
            json_obj.get("start").unwrap().as_i64().unwrap(),
            json_obj.get("end").unwrap().as_i64().unwrap(),
        ),
        // end_time: json_obj.get("end").unwrap().as_i64().unwrap(),
        quote_volume: None,
        json: msg.to_string(),
    };


    unimplemented!();
}
// Parse start and end time from candle stick message and convert to string
pub(crate) fn parse_candlestick_period(start: i64, end: i64) -> String {
    let period = (end - start) as u64;
    return match period/60 {
        1 => "1m".to_string(),
        3 => "3m".to_string(),
        5 => "5m".to_string(),
        15 => "15m".to_string(),
        30 => "30m".to_string(),
        60 => "1h".to_string(),
        120 => "2h".to_string(),
        240 => "4h".to_string(),
        360 => "6h".to_string(),
        480 => "8h".to_string(),
        720 => "12h".to_string(),
        1440 => "1d".to_string(),
        2880 => "2d".to_string(),
        4320 => "3d".to_string(),
        5760 => "4d".to_string(),
        7200 => "5d".to_string(),
        8640 => "6d".to_string(),
        10080 => "1w".to_string(),
        _ => panic!("Unknown period {}", period),
        // 1y => "1y",
    };
    
    // period.to_string()
}



pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<RawOrderbookMsg>(msg).map_err(|_e| {
        SimpleError::new(format!("Failed to deserialize {} to RawOrderbookMsg", msg))
    })?;
    let symbol = ws_msg.topic.strip_prefix("orderBookL2_25.").unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
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
        seq_id: None,
        prev_seq_id: None,
        asks: Vec::new(),
        bids: Vec::new(),
        snapshot,
        json: msg.to_string(),
    };

    let raw_orders = match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            if snapshot {
                serde_json::from_value::<Vec<RawOrder>>(ws_msg.data.clone()).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to Vec<RawOrder>",
                        ws_msg.data
                    ))
                })?
            } else {
                let tmp = serde_json::from_value::<OrderbookDelta>(ws_msg.data.clone()).map_err(
                    |_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to OrderbookDelta",
                            ws_msg.data
                        ))
                    },
                )?;
                let mut v = Vec::<RawOrder>::new();
                v.extend(tmp.delete);
                v.extend(tmp.update);
                v.extend(tmp.insert);
                v
            }
        }
        MarketType::LinearSwap => {
            if snapshot {
                let tmp = serde_json::from_value::<LinearOrderbookSnapshot>(ws_msg.data.clone())
                    .map_err(|_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to LinearOrderbookSnapshot",
                            ws_msg.data
                        ))
                    })?;
                tmp.order_book
            } else {
                let tmp = serde_json::from_value::<OrderbookDelta>(ws_msg.data.clone()).map_err(
                    |_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to OrderbookDelta",
                            ws_msg.data
                        ))
                    },
                )?;
                let mut v = Vec::<RawOrder>::new();
                v.extend(tmp.delete);
                v.extend(tmp.update);
                v.extend(tmp.insert);
                v
            }
        }
        _ => {
            return Err(SimpleError::new(format!(
                "Unknown market_type {}",
                market_type
            )))
        }
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
