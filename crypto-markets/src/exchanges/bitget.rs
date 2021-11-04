use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, MarketType, Precision,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::InverseSwap => fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        MarketType::InverseSwap => fetch_inverse_swap_markets(),
        MarketType::LinearSwap => fetch_linear_swap_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

#[derive(Serialize, Deserialize)]
struct LeverageFilter {
    min_leverage: i64,
    max_leverage: i64,
    leverage_step: String,
}

#[derive(Serialize, Deserialize)]
struct PriceFilter {
    min_price: String,
    max_price: String,
    tick_size: String,
}

#[derive(Serialize, Deserialize)]
struct LotSizeFilter {
    max_trading_qty: i64,
    min_trading_qty: f64,
    qty_step: f64,
}

// See https://github.com/BitgetLimited/API_Docs_en/wiki/REST_api_reference#get-datav1commonsymbols--query-all-trading-pairs-and-accuracy-supported-in-the-station
#[derive(Serialize, Deserialize)]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    tick_size: String,
    size_increment: String,
    status: String,
    base_asset_precision: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    ts: i64,
    data: Vec<SpotMarket>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    underlying_index: String,
    quote_currency: String,
    coin: String,
    contract_val: String,
    size_increment: String,
    tick_size: String,
    forwardContractFlag: bool,
    priceEndStep: i64,
    minLeverage: i64,
    maxLeverage: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.bitget.com/data/v1/common/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.status != "ok" {
        Err(Error(txt))
    } else {
        Ok(resp
            .data
            .into_iter()
            .filter(|m| m.status == "online")
            .collect())
    }
}

fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get("https://capi.bitget.com/api/swap/v3/market/contracts", None)?;
    let markets = serde_json::from_str::<Vec<SwapMarket>>(&txt)?;
    Ok(markets)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}

fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| !m.forwardContractFlag)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.forwardContractFlag)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&m.symbol, "bitget").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "bitget".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol,
                base_id: m.base_currency,
                quote_id: m.quote_currency,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: true,
                margin: false,
                // see https://www.bitget.com/en/rate?tab=1
                fees: Fees {
                    maker: 0.002,
                    taker: 0.002,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.tick_size.parse::<u32>().unwrap()) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.size_increment.parse::<u32>().unwrap()) as f64),
                },
                quantity_limit: None,
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect();
    Ok(markets)
}

fn to_market(raw_market: &SwapMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.symbol, "bitget").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    Market {
        exchange: "bitget".to_string(),
        market_type: if raw_market.forwardContractFlag {
            MarketType::LinearSwap
        } else {
            MarketType::InverseSwap
        },
        symbol: raw_market.symbol.to_string(),
        base_id: raw_market.underlying_index.to_string(),
        quote_id: raw_market.quote_currency.to_string(),
        settle_id: Some(raw_market.coin.to_string()),
        base,
        quote,
        settle: Some(raw_market.coin.to_string()),
        active: true,
        margin: true,
        // see https://www.bitget.com/en/rate?tab=1
        fees: Fees {
            maker: 0.0002,
            taker: 0.0006,
        },
        precision: Precision {
            tick_size: 1.0 / (10_i64.pow(raw_market.tick_size.parse::<u32>().unwrap()) as f64),
            lot_size: 1.0 / (10_i64.pow(raw_market.size_increment.parse::<u32>().unwrap()) as f64),
        },
        quantity_limit: None,
        contract_value: Some(raw_market.contract_val.parse::<f64>().unwrap()),
        delivery_date: None,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| !m.forwardContractFlag)
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.forwardContractFlag)
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
