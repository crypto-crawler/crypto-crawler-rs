use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

// see https://support.kraken.com/hc/en-us/articles/360022839491-API-URLs
const BASE_URL: &str = "https://futures.kraken.com/derivatives/api/v3";

/// The WebSocket client for Kraken Futures.
///
/// * REST API doc: <https://support.kraken.com/hc/en-us/sections/360003562331-REST-API-Public>
/// * Trading at: <https://futures.kraken.com/>
/// * Rate Limits: <https://support.kraken.com/hc/en-us/articles/360022635612-Request-Limits-REST-API->
///   * 500 every 10 seconds
pub struct KrakenFuturesRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl KrakenFuturesRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        KrakenFuturesRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get most recent trades.
    ///
    /// If `lastTime` is provided, return trade data from the specified
    /// lastTime.
    ///
    /// For example: <https://futures.kraken.com/derivatives/api/v3/history?symbol=PI_XBTUSD>
    #[allow(non_snake_case)]
    pub fn fetch_trades(symbol: &str, lastTime: Option<String>) -> Result<String> {
        gen_api!(format!("/history?symbol={}", symbol), lastTime)
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// For example: <https://futures.kraken.com/derivatives/api/v3/orderbook?symbol=PI_XBTUSD>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/orderbook?symbol={}", symbol))
    }
}
