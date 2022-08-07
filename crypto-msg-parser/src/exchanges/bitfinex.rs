use crate::{
    exchanges::utils::calc_quantity_and_volume, CandlestickMsg, Order, OrderBookMsg, TradeMsg,
    TradeSide,
};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

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
        Ok(key[pos + 1..].to_string())
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

// See <https://docs.bitfinex.com/reference/ws-public-trades>
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

// See <https://docs.bitfinex.com/reference/ws-public-books>
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

fn parse_one_candle(
    market_type: MarketType,
    symbol: &str,
    pair: &str,
    period: &str,
    nums: &[f64; 6],
) -> CandlestickMsg {
    let begin_time = nums[0] as i64;
    let open = nums[1];
    let close = nums[2];
    let high = nums[3];
    let low = nums[4];
    let volume = nums[5];

    CandlestickMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.to_string(),
        msg_type: MessageType::Candlestick,
        timestamp: begin_time,
        begin_time,
        open,
        high,
        low,
        close,
        volume,
        period: period.to_string(),
        quote_volume: None,
        json: "".to_string(),
    }
}

/// See <https://docs.bitfinex.com/reference/ws-public-candles>
///
/// Samples:
///
/// * `[{"key":"trade:5m:tBTCF0:USTF0","channel":"candles"},[1656649200000,19578,19599,19599,19560,1.51796395]]`
/// * `[{"key":"trade:1m:tBTCF0:USTF0","channel":"candles"},[[1656649380000,19590,19593,19593,19590,0.02779935],[1656649320000,19564,19588,19588,19560,0.7836714499999999]]]`
pub(crate) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<CandlestickMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<Vec<Value>>(msg).map_err(SimpleError::from)?;

    let (symbol, period) = {
        let key = ws_msg[0].as_object().unwrap()["key"]
            .as_str()
            .unwrap()
            .strip_prefix("trade:")
            .unwrap();
        let pos = key.find(':').unwrap();
        let period = &key[..pos];
        let symbol = &key[pos + 1..];
        (symbol, period)
    };

    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let data = ws_msg[1].clone();
    if data.as_array().unwrap().is_empty() {
        return Ok(vec![]);
    }

    let snapshot = {
        let arr = data.as_array().unwrap();
        arr[0].is_array()
    };

    if snapshot {
        let arr = data.as_array().unwrap();
        let candles: Vec<CandlestickMsg> = arr
            .iter()
            .map(|v| serde_json::from_value::<[f64; 6]>(v.clone()).unwrap())
            .map(|nums| parse_one_candle(market_type, symbol, &pair, period, &nums))
            .collect();
        Ok(candles)
    } else {
        let nums = serde_json::from_value::<[f64; 6]>(data).unwrap();
        let candlestick_msg = parse_one_candle(market_type, symbol, &pair, period, &nums);
        Ok(vec![candlestick_msg])
    }
}
