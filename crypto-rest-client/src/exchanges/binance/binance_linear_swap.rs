use super::super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://fapi.binance.com";

/// Binance USDT-margined Perpetual Swap market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/futures/en/>
///   * Trading at: <https://www.binance.com/en/futures/BTC_USDT>
pub struct BinanceLinearSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BinanceLinearSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BinanceLinearSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get compressed, aggregate trades.
    ///
    /// Equivalent to `/fapi/v1/aggTrades` with `limit=1000`
    ///
    /// For example: <https://fapi.binance.com/fapi/v1/aggTrades?symbol=BTCUSDT&limit=1000>
    pub fn fetch_agg_trades(
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String, reqwest::Error> {
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api!(
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
    /// For example: <https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api!("/fapi/v1/depth", symbol, limit)
    }
}
