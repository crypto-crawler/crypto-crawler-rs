use super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api-pub.bitfinex.com";

/// The REST client for Bitfinex, including all markets.
///
/// * REST API doc: <https://docs.bitfinex.com/docs/rest-general>
/// * Spot: <https://trading.bitfinex.com/trading>
/// * Swap: <https://trading.bitfinex.com/t/BTCF0:USTF0>
/// * Funding: <https://trading.bitfinex.com/funding>
pub struct BitfinexRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitfinexRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitfinexRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// /v2/trades/Symbol/hist
    pub fn fetch_trades(
        symbol: &str,
        limit: Option<u16>,
        start: Option<u64>,
        end: Option<u64>,
        sort: Option<i8>,
    ) -> Result<String, reqwest::Error> {
        gen_api!(
            format!("/v2/trades/{}/hist", symbol),
            limit,
            start,
            end,
            sort
        )
    }

    /// Get a L2 snapshot of orderbook.
    ///
    /// Equivalent to `/v2/book/Symbol/P0` with `len=100`
    ///
    /// For example: <https://api-pub.bitfinex.com/v2/book/tBTCUSD/P0?len=100>
    /// /v2/book/Symbol/Precision
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        let len = Some(100);
        gen_api!(format!("/v2/book/{}/{}", symbol, "P0"), len)
    }

    /// Get a L3 snapshot of orderbook.
    ///
    /// Equivalent to `/v2/book/Symbol/R0` with `len=100`
    ///
    /// For example: <https://api-pub.bitfinex.com/v2/book/tBTCUSD/R0?len=100>
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        let len = Some(100);
        gen_api!(format!("/v2/book/{}/R0", symbol), len)
    }
}
