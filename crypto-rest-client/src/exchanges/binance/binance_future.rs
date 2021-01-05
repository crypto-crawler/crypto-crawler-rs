use super::super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://dapi.binance.com";

/// Binance Coin-margined Future market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_quarter>
pub struct BinanceFutureRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BinanceFutureRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BinanceFutureRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get compressed, aggregate trades.
    ///
    /// Equivalent to `/dapi/v1/aggTrades` with `limit=1000`
    ///
    /// For example: <https://dapi.binance.com/dapi/v1/aggTrades?symbol=BTCUSD_210625&limit=1000>
    pub fn fetch_agg_trades(
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String, reqwest::Error> {
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api!(
            "/dapi/v1/aggTrades",
            symbol,
            from_id,
            start_time,
            end_time,
            limit
        )
    }

    /// Get a L2 snapshot of orderbook.
    ///
    /// Equivalent to `/dapi/v1/depth` with `limit=1000`
    ///
    /// For example: <https://dapi.binance.com/dapi/v1/depth?symbol=BTCUSD_210625&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api!("/dapi/v1/depth", symbol, limit)
    }
}
