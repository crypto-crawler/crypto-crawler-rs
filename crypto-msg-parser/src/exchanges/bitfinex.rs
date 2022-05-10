use crate::{exchanges::utils::calc_quantity_and_volume, Order, OrderBookMsg, TradeMsg, TradeSide};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use if_chain::if_chain;

use serde_json::Value;
use simple_error::SimpleError;

const EXCHANGE_NAME: &str = "bitfinex";

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    if_chain! {
        if let Some(obj) = arr[0].as_object();
        if let Some(symbol) = obj["symbol"].as_str();
        then {
            Ok(symbol.to_string())
        } else {
            Err(SimpleError::new(format!("Failed to extract symbol from {}", msg)))
        }
    }
}

pub(crate) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    if_chain! {
        if let Some(obj) = arr[0].as_object();
        if let Some(channel) = obj["channel"].as_str();
        then {
            match channel {
                "trades" => {
                    // see https://docs.bitfinex.com/reference#ws-public-trades
                    if arr[1].is_string() {
                        if let Some(timestamp) = arr[2].as_array().unwrap()[1].as_i64() {
                            Ok(Some(timestamp))
                        } else {
                            Err(SimpleError::new(format!("Failed to extract timestamp from {}", msg)))
                        }
                    } else if arr[1].is_array() {
                        // snapshot
                        let raw_trades: Vec<Vec<f64>> = serde_json::from_value(arr[1].clone()).unwrap();
                        let timestamp = raw_trades.iter().fold(std::i64::MIN, |a, raw_trade| {
                            a.max(raw_trade[1] as i64)
                        });
                        if timestamp == std::i64::MIN {
                            Err(SimpleError::new(format!("array is empty in {}", msg)))
                        } else {
                            Ok(Some(timestamp))
                        }
                    } else {
                        Err(SimpleError::new(format!("Failed to extract timestamp from {}", msg)))
                    }
                }
                "book" => Ok(None),
                _ => Err(SimpleError::new(format!("Failed to extract timestamp from {}", msg)))
            }
        } else {
            Err(SimpleError::new(format!("No channel field in {}", msg)))
        }
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
    let symbol = if_chain! {
        if let Some(obj) = arr[0].as_object();
        if let Some(symbol) = obj["symbol"].as_str();
        then {
            symbol
        } else {
            return Err(SimpleError::new(format!("Failed to extract symbol from {}", msg)));
        }
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
