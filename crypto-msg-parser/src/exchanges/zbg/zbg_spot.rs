use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde_json::Result;

const EXCHANGE_NAME: &str = "zbg";

// https://zbgapi.github.io/docs/spot/v1/en/#market-trade
// [T, symbol-id, symbol, timestamp, ask/bid, price, quantity]
pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let arr = serde_json::from_str::<Vec<Vec<String>>>(msg)?;

    let trades: Vec<TradeMsg> = arr
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
                quantity,
                volume: price * quantity,
                side,
                trade_id: timestamp.to_string(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}
