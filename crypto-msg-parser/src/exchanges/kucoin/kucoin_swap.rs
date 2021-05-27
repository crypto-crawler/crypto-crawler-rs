use crypto_market_type::MarketType;

use crate::{exchanges::utils::calc_quantity_and_volume, MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kucoin";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    baseCurrency: String,
    multiplier: f64,
    isInverse: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResponseMsg {
    code: String,
    data: Vec<SwapMarket>,
}

// https://docs.kucoin.cc/futures/#execution-data
#[derive(Serialize, Deserialize)]
struct ContractTradeMsg {
    symbol: String,
    sequence: i64,
    side: String, // buy, sell
    size: f64,
    price: f64,
    ts: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    subject: String,
    topic: String,
    #[serde(rename = "type")]
    type_: String,
    data: T,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<ContractTradeMsg>>(msg)?;
    let raw_trade = ws_msg.data;
    let pair = crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap();
    let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        raw_trade.price,
        raw_trade.size,
    );

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.symbol.clone(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.ts / 1000000,
        price: raw_trade.price,
        quantity_base,
        quantity_quote,
        quantity_contract,
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.sequence.to_string(),
        raw: serde_json::to_value(&raw_trade).unwrap(),
    };

    Ok(vec![trade])
}
