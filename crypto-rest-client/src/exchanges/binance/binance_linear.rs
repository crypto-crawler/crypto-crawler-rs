use super::super::utils::http_get;
use super::utils::*;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://fapi.binance.com";

/// Binance USDT-margined Future and Swap market.
///
/// * REST API doc: <https://binance-docs.github.io/apidocs/futures/en/>
/// * Trading at: <https://www.binance.com/en/futures/BTC_USDT>
/// * Rate Limits: <https://binance-docs.github.io/apidocs/futures/en/#limits>
///   * 2400 request weight per minute
pub struct BinanceLinearRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BinanceLinearRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BinanceLinearRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get compressed, aggregate trades.
    ///
    /// Equivalent to `/fapi/v1/aggTrades` with `limit=1000`
    ///
    /// For example:
    ///
    /// - <https://fapi.binance.com/fapi/v1/aggTrades?symbol=BTCUSDT&limit=1000>
    /// - <https://fapi.binance.com/fapi/v1/aggTrades?symbol=BTCUSDT_210625&limit=1000>
    pub fn fetch_agg_trades(
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String> {
        check_symbol(symbol);
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!(
            "/fapi/v1/aggTrades",
            symbol,
            from_id,
            start_time,
            end_time,
            limit
        )
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/fapi/v1/depth` with `limit=1000`
    ///
    /// For example:
    ///
    /// - <https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000>
    /// - <https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT_211231&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        check_symbol(symbol);
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!("/fapi/v1/depth", symbol, limit)
    }

    /// Get open interest.
    ///
    /// For example:
    ///
    /// - <https://fapi.binance.com/fapi/v1/openInterest?symbol=BTCUSDT>
    /// - <https://fapi.binance.com/fapi/v1/openInterest?symbol=BTCUSDT_211231>
    pub fn fetch_open_interest(symbol: &str) -> Result<String> {
        check_symbol(symbol);
        let symbol = Some(symbol);
        gen_api_binance!("/fapi/v1/openInterest", symbol)
    }
}
