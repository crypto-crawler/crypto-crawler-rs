use crypto_market_type::MarketType;

use super::utils::http_get;
use crate::{MessageType, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bitget";

lazy_static! {
    // See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
    static ref CONTRACT_VAL_MAPPING: HashMap<String, f64> = fetch_contract_val();
}

fn fetch_contract_val() -> HashMap<String, f64> {
    // See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SwapMarket {
        symbol: String,
        contract_val: String,
    }

    let txt = http_get("https://capi.bitget.com/api/swap/v3/market/contracts").unwrap();
    let swap_markets = serde_json::from_str::<Vec<SwapMarket>>(&txt).unwrap();

    let mut mapping: HashMap<String, f64> = HashMap::new();
    for swap_market in swap_markets.iter() {
        mapping.insert(
            swap_market.symbol.clone(),
            swap_market.contract_val.parse::<f64>().unwrap(),
        );
    }

    mapping
}

// see https://bitgetlimited.github.io/apidoc/en/swap/#public-trading-channel
#[derive(Serialize, Deserialize)]
struct SwapTradeMsg {
    instrument_id: String,
    price: String,
    side: String, // buy, sell
    size: String,
    timestamp: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct ResponseMsg<T: Sized> {
    table: String,
    data: Vec<T>,
}

fn calc_quantity_and_volume(
    market_type: MarketType,
    symbol: &str,
    price: f64,
    size: f64,
) -> (f64, f64) {
    let contract_value = CONTRACT_VAL_MAPPING.get(symbol).unwrap();
    if market_type == MarketType::LinearSwap {
        let real_quantity = contract_value * size;
        (real_quantity, real_quantity * price)
    } else if market_type == MarketType::InverseSwap {
        let volume = contract_value * size;
        (volume / price, volume)
    } else {
        panic!("Unknown market_type {}", market_type);
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<ResponseMsg<SwapTradeMsg>>(&msg)?;
    let trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade.price.parse::<f64>().unwrap();
            let size = raw_trade.size.parse::<f64>().unwrap();
            let (quantity, volume) =
                calc_quantity_and_volume(market_type, &raw_trade.instrument_id, price, size);

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_id.clone(),
                pair: crypto_pair::normalize_pair(&raw_trade.instrument_id, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp.parse::<i64>().unwrap(),
                price,
                quantity,
                volume,
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                // Use timestamp as ID because bitget doesn't provide trade_id
                trade_id: raw_trade.timestamp.to_string(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}
