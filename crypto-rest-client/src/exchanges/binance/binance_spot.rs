use super::super::utils::http_get;
use super::utils::*;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.binance.com";

/// Binance Spot market.
///
///   * RESTful API doc: <https://binance-docs.github.io/apidocs/spot/en/>
///   * Trading at: <https://www.binance.com/en/trade/BTC_USDT>
pub struct BinanceSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BinanceSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BinanceSpotRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get compressed, aggregate trades.
    ///
    /// Equivalent to `/api/v3/aggTrades` with `limit=1000`
    ///
    /// For example: <https://api.binance.com/api/v3/aggTrades?symbol=BTCUSDT&limit=1000>
    pub fn fetch_agg_trades(
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String> {
        check_symbol(symbol)?;
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!(
            "/api/v3/aggTrades",
            symbol,
            from_id,
            start_time,
            end_time,
            limit
        )
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/api/v3/depth` with `limit=1000`
    ///
    /// For example: <https://api.binance.com/api/v3/depth?symbol=BTCUSDT&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        check_symbol(symbol)?;
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!("/api/v3/depth", symbol, limit)
    }
}
