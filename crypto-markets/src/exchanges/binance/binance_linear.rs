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
struct LinearSwapMarket {
    symbol: String,
    pair: String,
    contractType: String,
    deliveryDate: u64,
    onboardDate: u64,
    status: String,
    maintMarginPercent: String,
    requiredMarginPercent: String,
    baseAsset: String,
    quoteAsset: String,
    marginAsset: String,
    pricePrecision: i64,
    quantityPrecision: i64,
    baseAssetPrecision: i64,
    quotePrecision: i64,
    underlyingType: String,
    triggerProtect: String,
    filters: Vec<HashMap<String, Value>>,
    orderTypes: Vec<String>,
    timeInForce: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://binance-docs.github.io/apidocs/futures/en/#exchange-information>
fn fetch_linear_markets_raw() -> Result<Vec<LinearSwapMarket>> {
    let txt = binance_http_get("https://fapi.binance.com/fapi/v1/exchangeInfo")?;
    let resp = serde_json::from_str::<BinanceResponse<LinearSwapMarket>>(&txt)?;
    let symbols: Vec<LinearSwapMarket> = resp
        .symbols
        .into_iter()
        .filter(|m| m.status == "TRADING")
        .collect();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_linear_markets_raw()?
        .into_iter()
        .filter(|m| m.contractType == "PERPETUAL")
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_linear_markets_raw()?
        .into_iter()
        .filter(|m| m.contractType != "PERPETUAL")
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_linear_markets_raw()?;
    let markets = raw_markets
        .into_iter()
        .map(|m| {
            Market {
                exchange: "binance".to_string(),
                market_type: if m.contractType == "PERPETUAL" {
                    MarketType::LinearSwap
                } else {
                    MarketType::LinearFuture
                },
                symbol: m.symbol.clone(),
                base_id: m.baseAsset.clone(),
                quote_id: m.quoteAsset.clone(),
                settle_id: Some(m.marginAsset.clone()),
                base: m.baseAsset.clone(),
                quote: m.quoteAsset.clone(),
                settle: Some(m.marginAsset.clone()),
                active: m.status == "TRADING",
                margin: true,
                // see https://www.binance.com/en/fee/futureFee
                fees: Fees {
                    maker: 0.0002,
                    taker: 0.0004,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.pricePrecision as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.quantityPrecision as u32) as f64),
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
                contract_value: Some(1.0),
                delivery_date: if m.contractType == "PERPETUAL" {
                    None
                } else {
                    Some(m.deliveryDate)
                },
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

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_linear_markets()?;
    let swap_markets = markets
        .into_iter()
        .filter(|m| m.market_type == MarketType::LinearSwap)
        .collect();
    Ok(swap_markets)
}

pub(super) fn fetch_linear_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_linear_markets()?;
    let future_markets = markets
        .into_iter()
        .filter(|m| m.market_type == MarketType::LinearFuture)
        .collect();
    Ok(future_markets)
}
