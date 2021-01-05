use super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://www.bitmex.com/api/v1";

/// The REST client for BitMEX.
///
/// BitMEX has Swap and Future markets.
///
///   * REST API doc: <https://www.bitmex.com/api/explorer/>
///   * Trading at: <https://www.bitmex.com/app/trade/>
pub struct BitMEXRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitMEXRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitMEXRestClient {
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
    pub fn fetch_trades(
        symbol: &str,
        start_time: Option<String>,
    ) -> Result<String, reqwest::Error> {
        let symbol = Some(symbol);
        let startTime = start_time;
        gen_api!("/trade", symbol, startTime)
    }

    /// Get a full L2 snapshot of orderbook.
    ///
    /// Equivalent to `/orderBook/L2` with `depth=0`
    ///
    /// For example: <https://www.bitmex.com/api/v1/orderBook/L2?symbol=XBTUSD&depth=0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        let symbol = Some(symbol);
        let depth = Some(0);
        gen_api!("/orderBook/L2", symbol, depth)
    }
}
