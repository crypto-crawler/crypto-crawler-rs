use super::utils::http_get;
use crate::{
    error::{Error, Result},
    market::*,
    utils::calc_precision,
    Market, MarketType,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::InverseFuture => fetch_inverse_future_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        MarketType::InverseSwap => fetch_inverse_swap_symbols(),
        MarketType::LinearOption => fetch_option_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

fn check_code_in_body(resp: String) -> Result<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&resp);
    if obj.is_err() {
        return Ok(resp);
    }

    match obj.unwrap().get("code") {
        Some(code) => {
            if code.as_i64().unwrap() != 0 {
                Err(Error(resp))
            } else {
                Ok(resp)
            }
        }
        None => Ok(resp),
    }
}

fn binance_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(resp) => check_code_in_body(resp),
        Err(_) => ret,
    }
}

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

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct LinearSwapMarket {
    symbol: String,
    pair: String,
    contractType: String,
    deliveryDate: i64,
    onboardDate: i64,
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

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct LinearOptionMarket {
    id: i64,
    contractId: i64,
    underlying: String,
    quoteAsset: String,
    symbol: String,
    unit: i64,
    minQty: f64,
    maxQty: f64,
    priceScale: i64,
    quantityScale: i64,
    side: String,
    expiryDate: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://binance-docs.github.io/apidocs/spot/en/#exchange-information>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = binance_http_get("https://api.binance.com/api/v3/exchangeInfo")?;
    let resp = serde_json::from_str::<BinanceResponse<SpotMarket>>(&txt).unwrap();
    Ok(resp.symbols)
}

// see <https://binance-docs.github.io/apidocs/delivery/en/#exchange-information>
fn fetch_future_markets_raw() -> Result<Vec<FutureMarket>> {
    let txt = binance_http_get("https://dapi.binance.com/dapi/v1/exchangeInfo")?;
    let resp = serde_json::from_str::<BinanceResponse<FutureMarket>>(&txt).unwrap();
    Ok(resp.symbols)
}

// see <https://binance-docs.github.io/apidocs/futures/en/#exchange-information>
fn fetch_linear_swap_markets_raw() -> Result<Vec<LinearSwapMarket>> {
    let txt = binance_http_get("https://fapi.binance.com/fapi/v1/exchangeInfo")?;
    let resp = serde_json::from_str::<BinanceResponse<LinearSwapMarket>>(&txt).unwrap();
    Ok(resp.symbols)
}

fn fetch_linear_option_markets_raw() -> Result<Vec<LinearOptionMarket>> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct OptionData {
        timezone: String,
        serverTime: i64,
        optionContracts: Vec<Value>,
        optionAssets: Vec<Value>,
        optionSymbols: Vec<LinearOptionMarket>,
    }
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct BinanceOptionResponse {
        code: i64,
        msg: String,
        data: OptionData,
    }

    let txt =
        binance_http_get("https://voptions.binance.com/options-api/v1/public/exchange/symbols")?;
    let resp = serde_json::from_str::<BinanceOptionResponse>(&txt).unwrap();
    Ok(resp.data.optionSymbols)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter(|m| m.status == "TRADING" && m.isSpotTradingAllowed)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|m| m.contractStatus == "TRADING")
        .map(|m| m.symbol)
        .filter(|symbol| !symbol.ends_with("_PERP"))
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|m| m.contractStatus == "TRADING")
        .map(|m| m.symbol)
        .filter(|symbol| symbol.ends_with("_PERP"))
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_linear_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.status == "TRADING" && m.contractType == "PERPETUAL")
        .map(|m| m.symbol)
        .filter(|symbol| symbol.ends_with("USDT"))
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_option_symbols() -> Result<Vec<String>> {
    let symbols = fetch_linear_option_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn parse_filter(filters: &[HashMap<String, Value>], filter_type: &str, field: &str) -> f64 {
    filters
        .iter()
        .find(|x| x["filterType"] == filter_type)
        .unwrap()[field]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap()
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_spot_markets_raw()?;
    let markets = raw_markets
        .into_iter()
        .map(|m| {
            Market {
                exchange: "binance".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol.clone(),
                pair: format!("{}/{}", m.baseAsset, m.quoteAsset),
                base: m.baseAsset.clone(),
                quote: m.quoteAsset.clone(),
                base_id: m.baseAsset.clone(),
                quote_id: m.quoteAsset.clone(),
                active: m.status == "TRADING" && m.isSpotTradingAllowed,
                margin: m.isMarginTradingAllowed,
                // see https://www.binance.com/en/fee/trading
                fees: Fees {
                    maker: 0.001,
                    taker: 0.001,
                },
                precision: Precision {
                    price: calc_precision(parse_filter(&m.filters, "PRICE_FILTER", "tickSize")),
                    base: calc_precision(parse_filter(&m.filters, "LOT_SIZE", "stepSize")),
                    quote: None,
                },
                min_quantity: MinQuantity {
                    base: Some(parse_filter(&m.filters, "LOT_SIZE", "minQty")),
                    quote: Some(parse_filter(&m.filters, "MIN_NOTIONAL", "minNotional")),
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
