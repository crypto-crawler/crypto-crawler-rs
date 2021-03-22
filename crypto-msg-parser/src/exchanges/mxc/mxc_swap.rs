use crypto_market_type::MarketType;

use super::super::utils::http_get;
use crate::{MessageType, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "mxc";

lazy_static! {
    // symbol -> contractSize
    static ref LINEAR_CONTRACT_VALUE_MAP: HashMap<String, f64> = fetch_linear_contract_sizes();
}

// get the contractSize field from linear markets
fn fetch_linear_contract_sizes() -> HashMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SwapMarket {
        symbol: String,
        baseCoin: String,
        quoteCoin: String,
        settleCoin: String,
        contractSize: f64,
    }

    #[derive(Serialize, Deserialize)]
    struct ResponseMsg {
        success: bool,
        code: i64,
        data: Vec<SwapMarket>,
    }

    let mut mapping: HashMap<String, f64> = HashMap::new();

    let txt = http_get("https://contract.mxc.com/api/v1/contract/detail").unwrap();
    let resp = serde_json::from_str::<ResponseMsg>(&txt).unwrap();
    for linear_market in resp.data.iter().filter(|x| x.settleCoin == x.quoteCoin) {
        mapping.insert(linear_market.symbol.clone(), linear_market.contractSize);
    }

    mapping
}

// https://mxcdevelop.github.io/APIDoc/contract.api.cn.html#4483df6e28
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    p: f64, // price
    v: f64, // quantity
    T: i64, // 1, buy; 2, sell
    t: i64, // timestamp
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    symbol: String,
    ts: i64,
    data: T,
}

fn calc_quantity_and_volume(
    market_type: MarketType,
    symbol: &str,
    raw_trade: &RawTradeMsg,
) -> (f64, f64) {
    match market_type {
        MarketType::InverseSwap => {
            let contract_value = if symbol.starts_with("BTC_") {
                100.0
            } else {
                10.0
            };
            let volume = raw_trade.v * contract_value;
            (volume / raw_trade.p, volume)
        }
        MarketType::LinearSwap => {
            let contract_value = LINEAR_CONTRACT_VALUE_MAP.get(symbol).unwrap();
            let quantity = raw_trade.v * contract_value;
            (quantity, raw_trade.p * quantity)
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}

pub(super) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg)?;
    let symbol = ws_msg.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_trade = ws_msg.data;

    let (quantity, volume) = calc_quantity_and_volume(market_type, symbol, &raw_trade);

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.t,
        price: raw_trade.p,
        quantity,
        volume,
        side: if raw_trade.T == 2 {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.t.to_string(),
        raw: serde_json::to_value(&raw_trade).unwrap(),
    };

    Ok(vec![trade])
}
