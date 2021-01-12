use super::utils::http_get;
use crate::error::Result;
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

    /// Get active trading symbols.
    ///
    /// For example: <https://api-pub.bitfinex.com/v2/conf/pub:list:pair:exchange>
    pub fn fetch_spot_symbols() -> Result<Vec<String>> {
        let text = gen_api!("/v2/conf/pub:list:pair:exchange")?;
        let symbols = serde_json::from_str::<Vec<Vec<String>>>(&text).unwrap();
        Ok(symbols[0].clone())
    }

    /// Get active trading symbols.
    ///
    /// For example: <https://api-pub.bitfinex.com/v2/conf/pub:list:pair:futures>
    pub fn fetch_swap_symbols() -> Result<Vec<String>> {
        let text = gen_api!("/v2/conf/pub:list:pair:futures")?;
        let symbols = serde_json::from_str::<Vec<Vec<String>>>(&text).unwrap();
        Ok(symbols[0].clone())
    }

    /// /v2/trades/Symbol/hist
    pub fn fetch_trades(
        symbol: &str,
        limit: Option<u16>,
        start: Option<u64>,
        end: Option<u64>,
        sort: Option<i8>,
    ) -> Result<String> {
        gen_api!(
            format!("/v2/trades/{}/hist", symbol),
            limit,
            start,
            end,
            sort
        )
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/v2/book/Symbol/P0` with `len=100`
    ///
    /// For example: <https://api-pub.bitfinex.com/v2/book/tBTCUSD/P0?len=100>
    /// /v2/book/Symbol/Precision
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let len = Some(100);
        gen_api!(format!("/v2/book/{}/{}", symbol, "P0"), len)
    }

    /// Get a Level3 snapshot of orderbook.
    ///
    /// Equivalent to `/v2/book/Symbol/R0` with `len=100`
    ///
    /// For example: <https://api-pub.bitfinex.com/v2/book/tBTCUSD/R0?len=100>
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String> {
        let len = Some(100);
        gen_api!(format!("/v2/book/{}/R0", symbol), len)
    }
}
