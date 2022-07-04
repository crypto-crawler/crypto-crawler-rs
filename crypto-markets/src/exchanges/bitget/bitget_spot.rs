use std::collections::HashMap;

use super::{super::utils::http_get, EXCHANGE_NAME};
use crate::{
    error::{Error, Result},
    Fees, Market, Precision, QuantityLimit,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// See https://bitgetlimited.github.io/apidoc/en/spot/#get-all-instruments
#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    symbol: String,         // symbol Id
    symbolName: String,     // symbol name
    baseCoin: String,       // Base coin
    quoteCoin: String,      // Denomination coin
    minTradeAmount: String, // Min trading amount
    maxTradeAmount: String, // Max trading amount
    takerFeeRate: String,   // Taker transaction fee rate
    makerFeeRate: String,   // Maker transaction fee rate
    priceScale: String,     // Maker transaction fee rate
    quantityScale: String,  // Quantity scale
    status: String,         // Status
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    code: String,
    msg: String,
    data: Vec<SpotMarket>,
    requestTime: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://bitgetlimited.github.io/apidoc/en/spot/#get-all-instruments
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.bitget.com/api/spot/v1/public/products", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.msg != "success" {
        Err(Error(txt))
    } else {
        let markets = resp
            .data
            .into_iter()
            // Ignored ETH_SPBL and BTC_SPBL for now because they're not tradable
            .filter(|x| x.status == "online" && x.symbol.ends_with("USDT_SPBL"))
            .collect::<Vec<SpotMarket>>();
        Ok(markets)
    }
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| Market {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::Spot,
            symbol: m.symbol.clone(),
            base_id: m.baseCoin.clone(),
            quote_id: m.quoteCoin.clone(),
            settle_id: None,
            base: m.baseCoin.clone(),
            quote: m.quoteCoin.clone(),
            settle: None,
            active: m.status == "online",
            margin: false,
            fees: Fees {
                maker: m.makerFeeRate.parse::<f64>().unwrap(),
                taker: m.takerFeeRate.parse::<f64>().unwrap(),
            },
            precision: Precision {
                tick_size: 1.0 / (10_i64.pow(m.priceScale.parse::<u32>().unwrap()) as f64),
                lot_size: 1.0 / (10_i64.pow(m.quantityScale.parse::<u32>().unwrap()) as f64),
            },
            quantity_limit: Some(QuantityLimit {
                min: m.minTradeAmount.parse::<f64>().ok(),
                max: if m.maxTradeAmount.parse::<f64>().unwrap() > 0.0 {
                    Some(m.maxTradeAmount.parse::<f64>().unwrap())
                } else {
                    None
                },
                notional_min: None,
                notional_max: None,
            }),
            contract_value: None,
            delivery_date: None,
            info: serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        })
        .collect();
    Ok(markets)
}
