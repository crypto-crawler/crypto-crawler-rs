use std::collections::HashMap;

use super::utils::http_get;
use crate::{error::Result, Fees, Market, MarketType, Precision, QuantityLimit};

use chrono::{prelude::*, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::InverseSwap => fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        MarketType::InverseFuture => fetch_inverse_future_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::InverseSwap => fetch_inverse_swap_markets(),
        MarketType::LinearSwap => fetch_linear_swap_markets(),
        MarketType::InverseFuture => fetch_inverse_future_markets(),
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
    max_trading_qty: f64,
    min_trading_qty: f64,
    qty_step: f64,
}

#[derive(Serialize, Deserialize)]
struct BybitMarket {
    name: String,
    alias: String,
    status: String,
    base_currency: String,
    quote_currency: String,
    price_scale: i64,
    taker_fee: String,
    maker_fee: String,
    leverage_filter: LeverageFilter,
    price_filter: PriceFilter,
    lot_size_filter: LotSizeFilter,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    ret_code: i64,
    ret_msg: String,
    ext_code: String,
    ext_info: String,
    result: Vec<BybitMarket>,
}

// See https://bybit-exchange.github.io/docs/inverse/#t-querysymbol
fn fetch_markets_raw() -> Result<Vec<BybitMarket>> {
    let txt = http_get("https://api.bybit.com/v2/public/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    assert_eq!(resp.ret_code, 0);
    Ok(resp
        .result
        .into_iter()
        .filter(|m| m.status == "Trading")
        .collect())
}

fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_markets_raw()?
        .into_iter()
        .filter(|m| m.name == m.alias && m.quote_currency == "USD")
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_markets_raw()?
        .into_iter()
        .filter(|m| m.name == m.alias && m.quote_currency == "USDT")
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_markets_raw()?
        .into_iter()
        .filter(|m| {
            m.quote_currency == "USD" && m.name[(m.name.len() - 2)..].parse::<i64>().is_ok()
        })
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &BybitMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.name, "bybit").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let delivery_date: Option<u64> = if raw_market.name[(raw_market.name.len() - 2)..]
        .parse::<i64>()
        .is_ok()
    {
        let n = raw_market.alias.len();
        let s = raw_market.alias.as_str();
        let month = &s[(n - 4)..(n - 2)];
        let day = &s[(n - 2)..];
        let now = Utc::now();
        let year = Utc::now().year();
        let delivery_time = DateTime::parse_from_rfc3339(
            format!("{}-{}-{}T00:00:00+00:00", year, month, day).as_str(),
        )
        .unwrap();
        let delivery_time = if delivery_time > now {
            delivery_time
        } else {
            DateTime::parse_from_rfc3339(
                format!("{}-{}-{}T00:00:00+00:00", year + 1, month, day).as_str(),
            )
            .unwrap()
        };
        assert!(delivery_time > now);
        Some(delivery_time.timestamp_millis() as u64)
    } else {
        None
    };
    Market {
        exchange: "bybit".to_string(),
        market_type: if raw_market.name == raw_market.alias {
            MarketType::InverseFuture
        } else if raw_market.quote_currency == "USDT" {
            MarketType::LinearSwap
        } else {
            MarketType::InverseSwap
        },
        symbol: raw_market.name.to_string(),
        base_id: raw_market.base_currency.to_string(),
        quote_id: raw_market.quote_currency.to_string(),
        settle_id: if raw_market.quote_currency == "USDT" {
            Some(raw_market.quote_currency.to_string())
        } else {
            Some(raw_market.base_currency.to_string())
        },
        base,
        quote,
        settle: if raw_market.quote_currency == "USDT" {
            Some(raw_market.quote_currency.to_string())
        } else {
            Some(raw_market.base_currency.to_string())
        },
        active: raw_market.status == "Trading",
        margin: true,
        fees: Fees {
            maker: raw_market.maker_fee.parse::<f64>().unwrap(),
            taker: raw_market.taker_fee.parse::<f64>().unwrap(),
        },
        precision: Precision {
            tick_size: raw_market.price_filter.tick_size.parse::<f64>().unwrap(),
            lot_size: raw_market.lot_size_filter.qty_step,
        },
        quantity_limit: Some(QuantityLimit {
            min: Some(raw_market.lot_size_filter.min_trading_qty),
            max: Some(raw_market.lot_size_filter.max_trading_qty),
            notional_min: None,
            notional_max: None,
        }),
        contract_value: Some(1.0),
        delivery_date,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_markets_raw()?
        .into_iter()
        .filter(|m| m.name == m.alias && m.quote_currency == "USD")
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_markets_raw()?
        .into_iter()
        .filter(|m| m.name == m.alias && m.quote_currency == "USDT")
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_markets_raw()?
        .into_iter()
        .filter(|m| {
            m.quote_currency == "USD" && m.name[(m.name.len() - 2)..].parse::<i64>().is_ok()
        })
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
