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
struct FutureMarket {
    symbol: String,
    pair: String,
    contractType: String,
    deliveryDate: i64,
    onboardDate: i64,
    contractStatus: String,
    contractSize: i64,
    marginAsset: String,
    maintMarginPercent: String,
    requiredMarginPercent: String,
    baseAsset: String,
    quoteAsset: String,
    pricePrecision: i64,
    quantityPrecision: i64,
    baseAssetPrecision: i64,
    quotePrecision: i64,
    equalQtyPrecision: i64,
    triggerProtect: String,
    underlyingType: String,
    filters: Vec<HashMap<String, Value>>,
    orderTypes: Vec<String>,
    timeInForce: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://binance-docs.github.io/apidocs/delivery/en/#exchange-information>
fn fetch_future_markets_raw() -> Result<Vec<FutureMarket>> {
    let txt = binance_http_get("https://dapi.binance.com/dapi/v1/exchangeInfo")?;
    let resp = serde_json::from_str::<BinanceResponse<FutureMarket>>(&txt)?;
    let symbols: Vec<FutureMarket> = resp
        .symbols
        .into_iter()
        .filter(|m| m.contractStatus == "TRADING")
        .collect();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|m| m.contractType != "PERPETUAL")
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|m| m.contractType == "PERPETUAL")
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_future_markets_internal() -> Result<Vec<Market>> {
    let raw_markets = fetch_future_markets_raw()?;
    let markets = raw_markets
        .into_iter()
        .map(|m| {
            Market {
                exchange: "binance".to_string(),
                market_type: if m.contractType == "PERPETUAL" {
                    MarketType::InverseSwap
                } else {
                    MarketType::InverseFuture
                },
                symbol: m.symbol.clone(),
                pair: format!("{}/{}", m.baseAsset, m.quoteAsset),
                base: m.baseAsset.clone(),
                quote: m.quoteAsset.clone(),
                base_id: m.baseAsset.clone(),
                quote_id: m.quoteAsset.clone(),
                active: m.contractStatus == "TRADING",
                margin: true,
                // see https://www.binance.com/en/fee/futureFee
                fees: Fees {
                    maker: 0.00015,
                    taker: 0.0004,
                },
                precision: Precision {
                    price: m.pricePrecision,
                    base: m.quantityPrecision,
                    quote: None,
                },
                min_quantity: MinQuantity {
                    base: Some(parse_filter(&m.filters, "LOT_SIZE", "minQty")),
                    quote: None,
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

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_internal()?
        .into_iter()
        .filter(|m| m.market_type == MarketType::InverseFuture)
        .collect();
    Ok(markets)
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_internal()?
        .into_iter()
        .filter(|m| m.market_type == MarketType::InverseSwap)
        .collect();
    Ok(markets)
}
