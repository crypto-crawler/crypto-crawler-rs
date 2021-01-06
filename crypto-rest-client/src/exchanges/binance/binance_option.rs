use super::super::utils::http_get;
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

    /// Get most recent trades.
    ///
    /// 500 recent trades are returned.
    ///
    /// For example: <https://voptions.binance.com/options-api/v1/public/market/trades?symbol=BTC-210129-40000-C&limit=500&t=1609956688000>
    pub fn fetch_trades(symbol: &str, start_time: Option<u64>) -> Result<String, reqwest::Error> {
        let t = start_time;
        gen_api!(
            format!("/public/market/trades?symbol={}&limit=500", symbol),
            t
        )
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/dapi/v1/depth` with `limit=1000`
    ///
    /// For example: <https://voptions.binance.com/options-api/v1/public/market/depth?symbol=BTC-210129-40000-C&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        gen_api!(format!("/public/market/depth?symbol={}&limit=1000", symbol))
    }
}
