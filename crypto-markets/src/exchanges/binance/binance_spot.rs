use super::utils::{binance_http_get, parse_filter};
use crate::{error::Result, market::*, Market, MarketType};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct BinanceResponse<T: Sized> {
    symbols: Vec<T>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    symbol: String,
    status: String,
    baseAsset: String,
    baseAssetPrecision: i32,
    quoteAsset: String,
    quotePrecision: i32,
    quoteAssetPrecision: i32,
    isSpotTradingAllowed: bool,
    isMarginTradingAllowed: bool,
    filters: Vec<HashMap<String, Value>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://binance-docs.github.io/apidocs/spot/en/#exchange-information>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = binance_http_get("https://api.binance.com/api/v3/exchangeInfo")?;
    let resp = serde_json::from_str::<BinanceResponse<SpotMarket>>(&txt)?;
    Ok(resp
        .symbols
        .into_iter()
        .filter(|s| s.symbol != "123456")
        .collect())
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter(|m| m.status == "TRADING" && m.isSpotTradingAllowed)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_spot_markets_raw()?;
    let markets = raw_markets
        .into_iter()
        .map(|m| {
            Market {
                exchange: "binance".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol.clone(),
                base_id: m.baseAsset.clone(),
                quote_id: m.quoteAsset.clone(),
                settle_id: None,
                base: m.baseAsset.clone(),
                quote: m.quoteAsset.clone(),
                settle: None,
                active: m.status == "TRADING" && m.isSpotTradingAllowed,
                margin: m.isMarginTradingAllowed,
                // see https://www.binance.com/en/fee/trading
                fees: Fees {
                    maker: 0.001,
                    taker: 0.001,
                },
                precision: Precision {
                    tick_size: parse_filter(&m.filters, "PRICE_FILTER", "tickSize")
                        .parse::<f64>()
                        .unwrap(),
                    lot_size: parse_filter(&m.filters, "LOT_SIZE", "stepSize")
                        .parse::<f64>()
                        .unwrap(),
                },
                quantity_limit: Some(QuantityLimit {
                    min: parse_filter(&m.filters, "LOT_SIZE", "minQty")
                        .parse::<f64>()
                        .ok(),
                    max: Some(
                        parse_filter(&m.filters, "LOT_SIZE", "maxQty")
                            .parse::<f64>()
                            .unwrap(),
                    ),
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
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
