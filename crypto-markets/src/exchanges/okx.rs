use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::Result,
    market::{Fees, Precision, QuantityLimit},
    Market,
};

// use chrono::DateTime;
use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::InverseFuture => fetch_inverse_future_symbols(),
        MarketType::LinearFuture => fetch_linear_future_symbols(),
        MarketType::InverseSwap => fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        MarketType::EuropeanOption => fetch_option_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        MarketType::InverseFuture => fetch_inverse_future_markets(),
        MarketType::LinearFuture => fetch_linear_future_markets(),
        MarketType::InverseSwap => fetch_inverse_swap_markets(),
        MarketType::LinearSwap => fetch_linear_swap_markets(),
        MarketType::EuropeanOption => fetch_option_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

// see <https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments>
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawMarket {
    instType: String,  // Instrument type
    instId: String,    // Instrument ID, e.g. BTC-USD-SWAP
    uly: String,       // Underlying, e.g. BTC-USD. Only applicable to FUTURES/SWAP/OPTION
    category: String,  // Fee schedule
    baseCcy: String,   // Base currency, e.g. BTC inBTC-USDT. Only applicable to SPOT
    quoteCcy: String,  // Quote currency, e.g. USDT in BTC-USDT. Only applicable to SPOT
    settleCcy: String, // Settlement and margin currency, e.g. BTC. Only applicable to FUTURES/SWAP/OPTION
    ctVal: String,     // Contract value. Only applicable to FUTURES/SWAP/OPTION
    ctMult: String,    // Contract multiplier. Only applicable to FUTURES/SWAP/OPTION
    ctValCcy: String,  // Contract value currency. Only applicable to FUTURES/SWAP/OPTION
    optType: String,   // Option type, C: Call P: put. Only applicable to OPTION
    stk: String,       // Strike price. Only applicable to OPTION
    listTime: String,  // Listing time, Unix timestamp format in milliseconds, e.g. 1597026383085
    expTime: String, // Expiry time, Unix timestamp format in milliseconds, e.g. 1597026383085. Only applicable to FUTURES/OPTION
    lever: String,   // Max Leverage. Not applicable to SPOT、OPTION
    tickSz: String,  // Tick size, e.g. 0.0001
    lotSz: String,   // Lot size, e.g. BTC-USDT-SWAP: 1
    minSz: String,   // Minimum order size
    ctType: String,  // Contract type, linear, inverse. Only applicable to FUTURES/SWAP
    alias: String, // Alias, this_week, next_week, quarter, next_quarter. Only applicable to FUTURES
    state: String, // Instrument status, live, suspend, preopen, settlement
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl RawMarket {
    fn to_market(&self) -> Market {
        let pair = crypto_pair::normalize_pair(self.instId.as_str(), "okx").unwrap();
        let (base, quote) = {
            let v: Vec<&str> = pair.split('/').collect();
            (v[0].to_string(), v[1].to_string())
        };
        let (market_type, base_id, quote_id) = if self.instType == "SPOT" {
            (
                MarketType::Spot,
                self.baseCcy.clone(),
                self.quoteCcy.clone(),
            )
        } else if self.instType == "FUTURES" {
            if self.ctType == "linear" {
                (
                    MarketType::LinearFuture,
                    self.ctValCcy.clone(),
                    self.settleCcy.clone(),
                )
            } else if self.ctType == "inverse" {
                (
                    MarketType::InverseFuture,
                    self.settleCcy.clone(),
                    self.ctValCcy.clone(),
                )
            } else {
                panic!("Unsupported ctType: {}", self.ctType);
            }
        } else if self.instType == "SWAP" {
            if self.ctType == "linear" {
                (
                    MarketType::LinearSwap,
                    self.ctValCcy.clone(),
                    self.settleCcy.clone(),
                )
            } else if self.ctType == "inverse" {
                (
                    MarketType::InverseSwap,
                    self.settleCcy.clone(),
                    self.ctValCcy.clone(),
                )
            } else {
                panic!("Unsupported ctType: {}", self.ctType);
            }
        } else if self.instType == "OPTION" {
            (
                MarketType::EuropeanOption,
                self.settleCcy.clone(),
                "USD".to_string(),
            )
        } else {
            panic!("Unsupported market_type: {}", self.instType);
        };

        Market {
            exchange: "okx".to_string(),
            market_type,
            symbol: self.instId.to_string(),
            base_id,
            quote_id,
            settle_id: if self.instType == "SPOT" {
                None
            } else {
                Some(self.settleCcy.clone())
            },
            base,
            quote,
            settle: if self.instType == "SPOT" {
                None
            } else {
                Some(self.settleCcy.clone())
            },
            active: self.state == "live",
            margin: !self.lever.is_empty(),
            // see https://www.okx.com/fees.html
            fees: Fees {
                maker: if self.instType == "SPOT" {
                    0.0008
                } else {
                    0.0002
                },
                taker: if self.instType == "SPOT" {
                    0.001
                } else {
                    0.0005
                },
            },
            precision: Precision {
                tick_size: self.tickSz.parse::<f64>().unwrap(),
                lot_size: self.lotSz.parse::<f64>().unwrap(),
            },
            quantity_limit: Some(QuantityLimit {
                min: self.minSz.parse::<f64>().ok(),
                max: None,
                notional_min: None,
                notional_max: None,
            }),
            contract_value: if self.instType == "SPOT" {
                None
            } else {
                Some(self.ctVal.parse::<f64>().unwrap())
            },
            delivery_date: if self.instType == "FUTURES" || self.instType == "OPTION" {
                Some(self.expTime.parse::<u64>().unwrap())
            } else {
                None
            },
            info: serde_json::to_value(self)
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        }
    }
}

// Retrieve a list of instruments.
//
// see <https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments>
// instType: SPOT, MARGIN, SWAP, FUTURES, OPTION
fn fetch_raw_markets_raw(inst_type: &str) -> Result<Vec<RawMarket>> {
    let markets = if inst_type == "OPTION" {
        let underlying_indexes = {
            let txt = http_get(
                "https://www.okx.com/api/v5/public/underlying?instType=OPTION",
                None,
            )?;
            let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
            let data = json_obj.get("data").unwrap().as_array().unwrap()[0]
                .as_array()
                .unwrap();
            data.iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        };

        let mut markets = Vec::<RawMarket>::new();
        for underlying in underlying_indexes.iter() {
            let url = format!(
                "https://www.okx.com/api/v5/public/instruments?instType=OPTION&uly={}",
                underlying
            );
            let txt = {
                let txt = http_get(url.as_str(), None)?;
                let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
                serde_json::to_string(json_obj.get("data").unwrap()).unwrap()
            };
            let mut arr = serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap();
            markets.append(&mut arr);
        }

        markets
    } else {
        let url = format!(
            "https://www.okx.com/api/v5/public/instruments?instType={}",
            inst_type
        );
        let txt = {
            let txt = http_get(url.as_str(), None)?;
            let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
            serde_json::to_string(json_obj.get("data").unwrap()).unwrap()
        };
        serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap()
    };
    Ok(markets.into_iter().filter(|x| x.state == "live").collect())
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_raw_markets_raw("SPOT")?
        .into_iter()
        .map(|m| m.instId)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_raw_markets_raw("FUTURES")?
        .into_iter()
        .filter(|m| m.ctType == "inverse")
        .map(|m| m.instId)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_raw_markets_raw("FUTURES")?
        .into_iter()
        .filter(|m| m.ctType == "linear")
        .map(|m| m.instId)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_raw_markets_raw("SWAP")?
        .into_iter()
        .filter(|m| m.ctType == "inverse")
        .map(|m| m.instId)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_raw_markets_raw("SWAP")?
        .into_iter()
        .filter(|m| m.ctType == "linear")
        .map(|m| m.instId)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_option_symbols() -> Result<Vec<String>> {
    let symbols = fetch_raw_markets_raw("OPTION")?
        .into_iter()
        .map(|m| m.instId)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_raw_markets_raw("SPOT")?
        .into_iter()
        .map(|m| m.to_market())
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_raw_markets_raw("FUTURES")?
        .into_iter()
        .filter(|m| m.ctType == "inverse")
        .map(|m| m.to_market())
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_linear_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_raw_markets_raw("FUTURES")?
        .into_iter()
        .filter(|m| m.ctType == "linear")
        .map(|m| m.to_market())
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_raw_markets_raw("SWAP")?
        .into_iter()
        .filter(|m| m.ctType == "inverse")
        .map(|m| m.to_market())
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_raw_markets_raw("SWAP")?
        .into_iter()
        .filter(|m| m.ctType == "linear")
        .map(|m| m.to_market())
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_option_markets() -> Result<Vec<Market>> {
    let markets = fetch_raw_markets_raw("OPTION")?
        .into_iter()
        .map(|m| m.to_market())
        .collect::<Vec<Market>>();
    Ok(markets)
}
