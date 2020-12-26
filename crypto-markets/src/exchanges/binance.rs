use crate::{
    market::{Fees, MinQuantity, Precision},
    utils::calc_precision,
    Market, MarketType,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
    filters: Vec<HashMap<String, Value>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FuturesMarket {
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
    filters: Vec<HashMap<String, Value>>,
    orderTypes: Vec<String>,
    timeInForce: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    status: String,
    maintMarginPercent: String,
    requiredMarginPercent: String,
    baseAsset: String,
    quoteAsset: String,
    pricePrecision: i64,
    quantityPrecision: i64,
    baseAssetPrecision: i64,
    quotePrecision: i64,
    filters: Vec<HashMap<String, Value>>,
    orderTypes: Vec<String>,
    timeInForce: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>, reqwest::Error> {
    match market_type {
        MarketType::Futures => fetch_futures_markets(),
        MarketType::Spot => fetch_spot_markets(),
        MarketType::Swap => fetch_swap_markets(),
        _ => panic!("Unknown market_type: {}", market_type),
    }
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

// https://github.com/binance/binance-spot-api-docs
// https://binance-docs.github.io/apidocs/spot/en/
fn fetch_spot_markets() -> Result<Vec<Market>, reqwest::Error> {
    let parsed = reqwest::blocking::get("https://api.binance.com/api/v3/exchangeInfo")?.json::<HashMap<String, Value>>()?;
    let symbols = serde_json::from_value::<Vec<SpotMarket>>(parsed["symbols"].clone()).unwrap();

    let transform = |pair: SpotMarket| -> Market {
        Market {
            exchange: "Binance".to_string(),
            market_type: MarketType::Spot,
            id: pair.symbol.clone(),
            pair: format!("{}_{}", pair.baseAsset, pair.quoteAsset),
            base: pair.baseAsset.clone(),
            quote: pair.quoteAsset.clone(),
            base_id: pair.baseAsset.clone(),
            quote_id: pair.quoteAsset.clone(),
            active: pair.status == "TRADING",
            // see https://www.binance.com/en/fee/trading
            fees: Fees {
                maker: 0.001,
                taker: 0.001,
            },
            precision: Precision {
                price: calc_precision(parse_filter(&pair.filters, "PRICE_FILTER", "tickSize")),
                base: calc_precision(parse_filter(&pair.filters, "LOT_SIZE", "stepSize")),
                quote: None,
            },
            min_quantity: MinQuantity {
                base: Some(parse_filter(&pair.filters, "LOT_SIZE", "minQty")),
                quote: Some(parse_filter(&pair.filters, "MIN_NOTIONAL", "minNotional")),
            },
            raw: serde_json::to_value(pair)
                .unwrap()
                .as_object()
                .unwrap()
                .into_iter()
                .map(|x| (x.0.clone(), x.1.clone()))
                .collect(),
        }
    };

    let result: Vec<Market> = symbols
        .into_iter()
        .filter(|m| m.isSpotTradingAllowed)
        .map(transform)
        .collect();
    Ok(result)
}

// https://binance-docs.github.io/apidocs/delivery/en/
fn fetch_futures_markets_internal() -> Result<Vec<Market>, reqwest::Error> {
    let parsed = reqwest::blocking::get("https://dapi.binance.com/dapi/v1/exchangeInfo")?.json::<HashMap<String, Value>>()?;
    let symbols = serde_json::from_value::<Vec<FuturesMarket>>(parsed["symbols"].clone()).unwrap();

    let transform = |pair: FuturesMarket| -> Market {
        Market {
            exchange: "Binance".to_string(),
            market_type: if pair.contractType == "PERPETUAL" {
                MarketType::Swap
            } else {
                MarketType::Futures
            },
            id: pair.symbol.clone(),
            pair: format!("{}_{}", pair.baseAsset, pair.quoteAsset),
            base: pair.baseAsset.clone(),
            quote: pair.quoteAsset.clone(),
            base_id: pair.baseAsset.clone(),
            quote_id: pair.quoteAsset.clone(),
            active: pair.contractStatus == "TRADING",
            // see https://www.binance.com/en/fee/futureFee
            fees: Fees {
                maker: 0.00015,
                taker: 0.0004,
            },
            precision: Precision {
                price: pair.pricePrecision,
                base: pair.quantityPrecision,
                quote: None,
            },
            min_quantity: MinQuantity {
                base: Some(parse_filter(&pair.filters, "LOT_SIZE", "minQty")),
                quote: None,
            },
            raw: serde_json::to_value(pair)
                .unwrap()
                .as_object()
                .unwrap()
                .into_iter()
                .map(|x| (x.0.clone(), x.1.clone()))
                .collect(),
        }
    };

    let result: Vec<Market> = symbols.into_iter().map(transform).collect();
    Ok(result)
}

fn fetch_futures_markets() -> Result<Vec<Market>, reqwest::Error> {
    let resp = fetch_futures_markets_internal();
    match resp {
        Ok(markets) => Ok(markets
            .into_iter()
            .filter(|m| m.market_type == MarketType::Futures)
            .collect::<Vec<Market>>()),
        Err(error) => Err(error),
    }
}

// https://binance-docs.github.io/apidocs/futures/en/
fn fetch_swap_markets() -> Result<Vec<Market>, reqwest::Error> {
    let parsed = reqwest::blocking::get("https://fapi.binance.com/fapi/v1/exchangeInfo")?.json::<HashMap<String, Value>>()?;
    let symbols = serde_json::from_value::<Vec<SwapMarket>>(parsed["symbols"].clone()).unwrap();

    let transform = |pair: SwapMarket| -> Market {
        Market {
            exchange: "Binance".to_string(),
            market_type: MarketType::Swap, // see https://binance.zendesk.com/hc/en-us/articles/360033524991-Differences-Between-a-Perpetual-Contract-and-a-Traditional-Futures-Contract
            id: pair.symbol.clone(),
            pair: format!("{}_{}", pair.baseAsset, pair.quoteAsset),
            base: pair.baseAsset.clone(),
            quote: pair.quoteAsset.clone(),
            base_id: pair.baseAsset.clone(),
            quote_id: pair.quoteAsset.clone(),
            active: pair.status == "TRADING",
            // see https://www.binance.com/en/fee/futureFee
            fees: Fees {
                maker: 0.0002,
                taker: 0.0004,
            },
            precision: Precision {
                price: pair.pricePrecision,
                base: pair.quantityPrecision,
                quote: None,
            },
            min_quantity: MinQuantity {
                base: Some(parse_filter(&pair.filters, "LOT_SIZE", "minQty")),
                quote: None,
            },
            raw: serde_json::to_value(pair)
                .unwrap()
                .as_object()
                .unwrap()
                .into_iter()
                .map(|x| (x.0.clone(), x.1.clone()))
                .collect(),
        }
    };

    let mut usdt_futures: Vec<Market> = symbols.into_iter().map(transform).collect();

    let mut coin_futures: Vec<Market> = fetch_futures_markets_internal()?
        .into_iter()
        .filter(|m| m.market_type == MarketType::Swap)
        .collect();
    usdt_futures.append(&mut coin_futures);
    Ok(usdt_futures)
}
