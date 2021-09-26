use super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://www.bitmex.com/api/v1";

/// The REST client for BitMEX.
///
/// BitMEX has Swap and Future markets.
///
/// * REST API doc: <https://www.bitmex.com/api/explorer/>
/// * Trading at: <https://www.bitmex.com/app/trade/>
/// * Rate Limits: <https://www.bitmex.com/app/restAPI#Limits>
///   * 60 requests per minute on all routes (reduced to 30 when unauthenticated)
///   * 10 requests per second on certain routes (see below)
pub struct BitmexRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitmexRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitmexRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get trades from a beginning time.
    ///
    /// Equivalent to `/trade` with `count=1000`
    ///
    /// For example: <https://www.bitmex.com/api/v1/trade?symbol=XBTUSD&count=1000&startTime=2018-09-28T12:34:25.706Z>
    #[allow(non_snake_case)]
    pub fn fetch_trades(symbol: &str, start_time: Option<String>) -> Result<String> {
        let symbol = Some(symbol);
        let startTime = start_time;
        gen_api!("/trade", symbol, startTime)
    }

    /// Get a full Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/orderBook/L2` with `depth=0`
    ///
    /// For example: <https://www.bitmex.com/api/v1/orderBook/L2?symbol=XBTUSD&depth=0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let symbol = Some(symbol);
        let depth = Some(0);
        gen_api!("/orderBook/L2", symbol, depth)
    }
}
