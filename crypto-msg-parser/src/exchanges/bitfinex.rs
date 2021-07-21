use crypto_market_type::MarketType;

use crate::{
    exchanges::utils::calc_quantity_and_volume, MessageType, Order, OrderBookMsg, TradeMsg,
    TradeSide,
};

use serde_json::{Result, Value};

const EXCHANGE_NAME: &str = "bitfinex";

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
        raw: serde_json::to_value(&nums).unwrap(),
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

    // see https://docs.bitfinex.com/reference#ws-public-trades
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

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<Vec<Value>>(&msg)?;

    let symbol = ws_msg[0]
        .as_object()
        .unwrap()
        .get("symbol")
        .unwrap()
        .as_str()
        .unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let data = ws_msg[1].clone();

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
        asks: Vec::new(),
        bids: Vec::new(),
        snapshot,
        raw: serde_json::from_str(msg)?,
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
