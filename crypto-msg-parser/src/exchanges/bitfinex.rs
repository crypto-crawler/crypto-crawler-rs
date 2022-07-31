use crate::{BboMsg,KlineMsg,exchanges::utils::calc_quantity_and_volume, Order, OrderBookMsg, TradeMsg, TradeSide};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};

use serde_json::Value;
use simple_error::SimpleError;

const EXCHANGE_NAME: &str = "bitfinex";

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    if arr.is_empty() {
        return Ok("NONE".to_string());
    }
    if !arr[0].is_object() {
        return Ok("NONE".to_string());
    }
    let obj = arr[0].as_object().unwrap();
    let channel = obj["channel"].as_str().unwrap();
    if let Some(symbol) = obj.get("symbol") {
        Ok(symbol.as_str().unwrap().to_string())
    } else if channel == "candles" {
        let key = &(obj["key"].as_str().unwrap())["trade:".len()..];
        let pos = key.find(':').unwrap();
        Ok((&key[pos + 1..]).to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    if arr.is_empty() {
        return Ok(None);
    }
    if !arr[0].is_object() {
        return Ok(None);
    }
    let obj = arr[0].as_object().unwrap();
    let channel = obj["channel"].as_str().unwrap();
    match channel {
        "trades" => {
            // see https://docs.bitfinex.com/reference#ws-public-trades
            if arr[1].is_string() {
                if let Some(timestamp) = arr[2].as_array().unwrap()[1].as_i64() {
                    Ok(Some(timestamp))
                } else {
                    Err(SimpleError::new(format!(
                        "Failed to extract timestamp from {}",
                        msg
                    )))
                }
            } else if arr[1].is_array() {
                // snapshot
                let raw_trades: Vec<Vec<f64>> = serde_json::from_value(arr[1].clone()).unwrap();
                let timestamp = raw_trades.iter().map(|raw_trade| raw_trade[1] as i64).max();
                Ok(timestamp) // Sometimes data can be empty, for example: [{"channel":"trades","symbol":"tBTC:CNHT"}, []]
            } else {
                Err(SimpleError::new(format!(
                    "Failed to extract timestamp from {}",
                    msg
                )))
            }
        }
        "candles" => {
            if let Ok(arr_2d) = serde_json::from_value::<Vec<Vec<f64>>>(arr[1].clone()) {
                let timestamp = arr_2d.iter().map(|v| v[0] as i64).max();
                Ok(timestamp)
            } else {
                let arr: Vec<f64> = serde_json::from_value(arr[1].clone()).unwrap();
                Ok(Some(arr[0] as i64))
            }
        }
        "book" | "ticker" => Ok(None),
        _ => Err(SimpleError::new(format!(
            "Failed to extract timestamp from {}",
            msg
        ))),
    }
}

fn parse_one_trade(market_type: MarketType, symbol: &str, nums: &[f64]) -> TradeMsg {
    assert_eq!(4, nums.len());
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let trade_id = nums[0] as i64;
    let timestamp = nums[1] as i64;
    let quantity = f64::abs(nums[2]);
    let price = nums[3];

    let (quantity_base, quantity_quote, quantity_contract) =
        calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

    TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::Trade,
        timestamp,
        price,
        quantity_base,
        quantity_quote,
        quantity_contract,
        side: if quantity < 0.0 {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: trade_id.to_string(),
        json: serde_json::to_string(&nums).unwrap(),
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    let obj = arr[0].as_object().unwrap();
    let symbol = if let Some(symbol) = obj.get("symbol") {
        symbol.as_str().unwrap()
    } else {
        return Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )));
    };

    // see https://docs.bitfinex.com/reference#ws-public-trades
    match arr[1].as_str() {
        Some(_) => {
            // te, tu
            let nums: Vec<f64> = serde_json::from_value(arr[2].clone()).unwrap();
            let mut trade = parse_one_trade(market_type, symbol, &nums);
            trade.json = msg.to_string();
            Ok(vec![trade])
        }
        None => {
            // snapshot
            let nums_arr: Vec<Vec<f64>> = serde_json::from_value(arr[1].clone()).unwrap();
            let mut trades: Vec<TradeMsg> = nums_arr
                .iter()
                .map(|nums| parse_one_trade(market_type, symbol, nums))
                .collect();
            if trades.len() == 1 {
                trades[0].json = msg.to_string();
            }
            Ok(trades)
        }
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;

    let symbol = ws_msg[0].as_object().unwrap()["symbol"].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let data = ws_msg[1].clone();
    if data.as_array().unwrap().is_empty() {
        return Ok(vec![]);
    }

    let snapshot = {
        let arr = data.as_array().unwrap();
        arr[0].is_array()
    };

    let parse_order = |x: &[f64; 3]| -> Order {
        let price = x[0];
        // delete price level if count = 0
        let quantity = if (x[1] as i32) == 0 {
            0.0
        } else {
            f64::abs(x[2])
        };

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
        pair: pair.clone(),
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: None,
        prev_seq_id: None,
        asks: Vec::new(),
        bids: Vec::new(),
        snapshot,
        json: msg.to_string(),
    };

    let raw_orders = if snapshot {
        // snapshot
        serde_json::from_value::<Vec<[f64; 3]>>(data).unwrap()
    } else {
        // update
        let raw_order = serde_json::from_value::<[f64; 3]>(data).unwrap();
        vec![raw_order]
    };
    for raw_order in raw_orders.iter() {
        let order = parse_order(raw_order);
        if raw_order[2] > 0.0 {
            orderbook.bids.push(order);
        } else {
            orderbook.asks.push(order);
        }
    }

    Ok(vec![orderbook])
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsg {
    len: String,
    prec: String,
    freq: String,
    symbol: String,
    channel: String,
}
// {"len":"1","prec":"R0","freq":"F0","symbol":"tBTCUST","channel":"book"}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawCandlesMsg {
    key: String,
    channel: String,
}
//{"key":"trade:1m:tBTCUST","channel":"candles"}
pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    let ws_msg = serde_json::from_str::<(RawBboMsg,Vec<Vec<f64>>)>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawBboMsg>",
            msg
        ))
    })?;




    // debug_assert!(ws_msg.stream.ends_with("bookTicker"));
    let timestamp = received_at.unwrap();



    let symbol = ws_msg.0.symbol;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.1[0][0],
        ws_msg.1[0][1],
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.1[1][0],
        ws_msg.1[1][1],
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price: ws_msg.1[0][0],
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.1[1][0],
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: None,
        json: msg.to_string(),
    };
    Ok(bbo_msg)
}

pub(crate) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
    msg_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    let obj = serde_json::from_str::<(RawCandlesMsg,Vec<f64>)>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    let tempKey:Vec<&str> = obj.0.key.split(":").collect();
    let symbol = tempKey[2].to_string();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();


    let open: f64 = obj.1[1];
    let high: f64 = obj.1[3];
    let low: f64 = obj.1[4];
    let close: f64 = obj.1[2];
    let volume: f64 = obj.1[5];
    // let quote_volume: f64 = obj.1[0][0];


    // obj.data.k

    let kline_msg = KlineMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type,
        timestamp: obj.1[0] as i64,
        json: msg.to_string(),
        open,
        high,
        low,
        close,
        volume,
        period: tempKey[1].to_string(),
        quote_volume: None
    };

    Ok(kline_msg)
}