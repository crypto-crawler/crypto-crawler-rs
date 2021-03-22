use crypto_market_type::MarketType;

use super::super::utils::http_get;
use crate::{MessageType, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kucoin";

lazy_static! {
    static ref LINEAR_MULTIPLIERS: HashMap<String, f64> = fetch_linear_multipliers();
}

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

// get the multiplier field from linear markets
fn fetch_linear_multipliers() -> HashMap<String, f64> {
    let mut mapping: HashMap<String, f64> = HashMap::new();

    let txt = http_get("https://api-futures.kucoin.com/api/v1/contracts/active").unwrap();
    let resp = serde_json::from_str::<ResponseMsg>(&txt).unwrap();
    for swap_market in resp.data.iter().filter(|x| !x.isInverse) {
        mapping.insert(swap_market.baseCurrency.to_string(), swap_market.multiplier);
    }

    mapping
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

fn calc_quantity_and_volume(market_type: MarketType, raw_trade: &ContractTradeMsg) -> (f64, f64) {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            // Each contract value is 1USD
            let volume = raw_trade.size;
            (volume / raw_trade.price, volume)
        }
        MarketType::LinearSwap => {
            let base_id = raw_trade.symbol.strip_suffix("USDTM").unwrap();
            let multiplier = LINEAR_MULTIPLIERS.get(base_id).unwrap();
            let quantity = raw_trade.size * multiplier;
            (quantity, raw_trade.price * quantity)
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<ContractTradeMsg>>(msg)?;
    let raw_trade = ws_msg.data;
    let (quantity, volume) = calc_quantity_and_volume(market_type, &raw_trade);

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.symbol.clone(),
        pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
        msg_type: MessageType::Trade,
        timestamp: raw_trade.ts / 1000000,
        price: raw_trade.price,
        quantity,
        volume,
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
