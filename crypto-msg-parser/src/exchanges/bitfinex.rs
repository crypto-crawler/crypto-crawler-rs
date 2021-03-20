use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde_json::{Result, Value};

const EXCHANGE_NAME: &str = "bitfinex";

fn parse_one_trade(market_type: MarketType, symbol: &str, nums: &[f64]) -> TradeMsg {
    assert_eq!(4, nums.len());
    let trade_id = nums[0] as i64;
    let timestamp = nums[1] as i64;
    let quantity = nums[2];
    let price = nums[3];

    TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap(),
        msg_type: MessageType::Trade,
        timestamp,
        price,
        quantity: f64::abs(quantity),
        side: if quantity < 0.0 {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: trade_id.to_string(),
        raw: serde_json::to_value(nums).unwrap(),
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let arr = serde_json::from_str::<Vec<Value>>(&msg)?;

    let symbol = arr[0]
        .as_object()
        .unwrap()
        .get("symbol")
        .unwrap()
        .as_str()
        .unwrap();

    match arr[1].as_str() {
        Some(_) => {
            // te, tu
            let nums: Vec<f64> = serde_json::from_value(arr[2].clone()).unwrap();
            let mut trade = parse_one_trade(market_type, symbol, &nums);
            trade.raw = serde_json::from_str(msg).unwrap();
            Ok(vec![trade])
        }
        None => {
            // snapshot
            let mut trades = Vec::<TradeMsg>::new();
            let nums_arr: Vec<Vec<f64>> = serde_json::from_value(arr[1].clone()).unwrap();
            for nums in nums_arr.iter() {
                let trade = parse_one_trade(market_type, symbol, &nums);
                trades.push(trade);
            }
            Ok(trades)
        }
    }
}
