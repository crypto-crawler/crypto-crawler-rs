use super::super::utils::http_get;
use super::utils::*;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://vapi.binance.com";

/// Binance Option market.
///
/// * REST API doc: <https://binance-docs.github.io/apidocs/voptions/en/>
/// * Trading at: <https://voptions.binance.com/en>
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

    /// Get most recent trades.
    ///
    /// 500 recent trades are returned.
    ///
    /// For example: <https://voptions.binance.com/options-api/v1/public/market/trades?symbol=BTC-210129-40000-C&limit=500&t=1609956688000>
    pub fn fetch_trades(symbol: &str, start_time: Option<u64>) -> Result<String> {
        check_symbol(symbol);
        let t = start_time;
        gen_api_binance!(format!("/vapi/v1/trades?symbol={}&limit=500", symbol), t)
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// For example: <https://vapi.binance.com/vapi/v1/depth?symbol=BTC-211001-30000-P&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        check_symbol(symbol);
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!("/vapi/v1/depth", symbol, limit)
    }
}
