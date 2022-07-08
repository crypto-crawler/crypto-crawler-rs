use std::collections::HashMap;

use super::utils::http_get;
use crate::{error::Result, Fees, Market, MarketType, Precision};

use chrono::{prelude::*, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        MarketType::LinearFuture => fetch_linear_future_symbols(),
        MarketType::Move => fetch_move_symbols(),
        MarketType::BVOL => fetch_bvol_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        MarketType::LinearSwap => fetch_linear_swap_markets(),
        MarketType::LinearFuture => fetch_linear_future_markets(),
        MarketType::Move => fetch_move_markets(),
        MarketType::BVOL => fetch_bvol_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FtxMarket {
    name: String,
    baseCurrency: Option<String>,
    quoteCurrency: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    underlying: Option<String>,
    enabled: bool,
    postOnly: bool,
    priceIncrement: f64,
    sizeIncrement: f64,
    restricted: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool,
    result: Vec<FtxMarket>,
}

fn fetch_markets_raw() -> Result<Vec<FtxMarket>> {
    let txt = http_get("https://ftx.com/api/markets", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    assert!(resp.success);
    let valid: Vec<FtxMarket> = resp.result.into_iter().filter(|x| x.enabled).collect();
    Ok(valid)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "spot" && !x.name.contains("BVOL/"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "future" && x.name.ends_with("-PERP"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| {
            x.type_ == "future"
                && !x.name.ends_with("-PERP")
                && !x.name.contains("-MOVE-")
                && x.name[(x.name.len() - 4)..].parse::<u32>().is_ok()
                && x.name.contains('-')
        })
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_move_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "future" && x.name.contains("-MOVE-"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_bvol_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "spot" && x.name.contains("BVOL/"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn to_market(raw_market: &FtxMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.name, "ftx").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let market_type = if raw_market.type_ == "spot" {
        if raw_market.name.contains("BVOL/") {
            MarketType::BVOL
        } else {
            MarketType::Spot
        }
    } else if raw_market.type_ == "future" {
        if raw_market.name.ends_with("-PERP") {
            MarketType::LinearSwap
        } else if raw_market.name.contains("-MOVE-") {
            MarketType::Move
        } else {
            MarketType::LinearFuture
        }
    } else {
        panic!("Unsupported type: {}", raw_market.type_);
    };
    let delivery_date: Option<u64> = if raw_market.name[(raw_market.name.len() - 4)..]
        .parse::<u32>()
        .is_ok()
    {
        let n = raw_market.name.len();
        let s = raw_market.name.as_str();
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
        exchange: "ftx".to_string(),
        market_type,
        symbol: raw_market.name.to_string(),
        base_id: if raw_market.type_ == "spot" {
            raw_market.baseCurrency.clone().unwrap()
        } else {
            raw_market.underlying.clone().unwrap()
        },
        quote_id: if raw_market.type_ == "spot" {
            raw_market.quoteCurrency.clone().unwrap()
        } else {
            "USD".to_string()
        },
        settle_id: if raw_market.type_ == "spot" {
            None
        } else {
            Some("USD".to_string())
        },
        base,
        quote,
        settle: if raw_market.type_ == "spot" {
            None
        } else {
            Some("USD".to_string())
        },
        active: raw_market.enabled,
        margin: true,
        // see https://help.ftx.com/hc/en-us/articles/360024479432-Fees
        fees: Fees {
            maker: 0.0002,
            taker: 0.0007,
        },
        precision: Precision {
            tick_size: raw_market.priceIncrement,
            lot_size: raw_market.sizeIncrement,
        },
        quantity_limit: None,
        contract_value: if raw_market.type_ == "spot" {
            None
        } else {
            Some(1.0)
        },
        delivery_date,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_markets_raw()?
        .into_iter()
        .filter(|x| x.type_ == "spot")
        .map(|x| to_market(&x))
        .collect();
    Ok(markets)
}

fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<Market> = markets
        .into_iter()
        .filter(|x| x.type_ == "future" && x.name.ends_with("-PERP"))
        .map(|x| to_market(&x))
        .collect();
    Ok(symbols)
}

fn fetch_linear_future_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_markets_raw()?
        .into_iter()
        .filter(|x| {
            x.type_ == "future"
                && !x.name.ends_with("-PERP")
                && !x.name.contains("-MOVE-")
                && x.name[(x.name.len() - 4)..].parse::<u32>().is_ok()
                && x.name.contains('-')
        })
        .map(|x| to_market(&x))
        .collect();
    Ok(markets)
}

fn fetch_move_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_markets_raw()?
        .into_iter()
        .filter(|x| x.type_ == "future" && x.name.contains("-MOVE-"))
        .map(|x| to_market(&x))
        .collect();
    Ok(markets)
}

fn fetch_bvol_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_markets_raw()?
        .into_iter()
        .filter(|x| x.type_ == "spot" && x.name.contains("BVOL/"))
        .map(|x| to_market(&x))
        .collect();
    Ok(markets)
}
