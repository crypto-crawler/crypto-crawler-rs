use serde_json::Value;

use super::super::utils::http_get;
use super::utils::*;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://voptions.binance.com/options-api/v1";

/// Binance Option market.
///
///   * REST API doc: None
///   * Trading at: <https://voptions.binance.com/en>
pub struct BinanceOptionRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BinanceOptionRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BinanceOptionRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get active trading symbols.
    pub fn fetch_symbols() -> Result<Vec<String>> {
        let txt = gen_api_binance!("/public/exchange/symbols")?;
        let obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
        if obj.get("code").unwrap().as_i64().unwrap() != 0 {
            return Err(crate::Error(txt));
        }

        let arr = obj
            .get("data")
            .unwrap()
            .as_object()
            .unwrap()
            .get("optionSymbols")
            .unwrap()
            .as_array()
            .unwrap();
        let symbols = arr
            .iter()
            .map(|x| x.as_object().unwrap())
            .map(|obj| obj.get("symbol").unwrap().as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        Ok(symbols)
    }

    /// Get most recent trades.
    ///
    /// 500 recent trades are returned.
    ///
    /// For example: <https://voptions.binance.com/options-api/v1/public/market/trades?symbol=BTC-210129-40000-C&limit=500&t=1609956688000>
    pub fn fetch_trades(symbol: &str, start_time: Option<u64>) -> Result<String> {
        check_symbol(symbol);
        let t = start_time;
        gen_api_binance!(
            format!("/public/market/trades?symbol={}&limit=500", symbol),
            t
        )
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/dapi/v1/depth` with `limit=1000`
    ///
    /// For example: <https://voptions.binance.com/options-api/v1/public/market/depth?symbol=BTC-210129-40000-C&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        check_symbol(symbol);
        gen_api_binance!(format!("/public/market/depth?symbol={}&limit=1000", symbol))
    }
}
