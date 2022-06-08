use super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.bybit.com/v2";

/// The RESTful client for Bybit.
///
/// Bybit has InverseSwap and LinearSwap markets.
///
/// * RESTful API doc: <https://bybit-exchange.github.io/docs/inverse/#t-marketdata>
/// * Trading at:
///     * InverseSwap <https://www.bybit.com/trade/inverse/>
///     * LinearSwap <https://www.bybit.com/trade/usdt/>
/// * Rate Limit: <https://bybit-exchange.github.io/docs/inverse/#t-ratelimits>
///   * GET method:
///     * 50 requests per second continuously for 2 minutes
///     * 70 requests per second continuously for 5 seconds
///   * POST method:
///     * 20 requests per second continuously for 2 minutes
///     * 50 requests per second continuously for 5 seconds
pub struct BybitRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BybitRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BybitRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 50 bids and asks are returned.
    ///
    /// For example: <https://api.bybit.com/v2/public/orderBook/L2?symbol=BTCUSD>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/public/orderBook/L2?symbol={}", symbol))
    }

    /// Get open interest.
    ///
    /// For example:
    ///
    /// - <https://api.bybit.com/v2/public/open-interest?symbol=BTCUSD&period=5min&limit=200>
    /// - <https://api.bybit.com/v2/public/open-interest?symbol=BTCUSDT&period=5min&limit=200>
    /// - <https://api.bybit.com/v2/public/open-interest?symbol=BTCUSDU22&period=5min&limit=200>
    pub fn fetch_open_interest(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/public/open-interest?symbol={}&period=5min&limit=200",
            symbol
        ))
    }

    /// Get long-short ratio.
    ///
    /// For example:
    ///
    /// - <https://api.bybit.com/v2/public/account-ratio?symbol=BTCUSD&period=5min&limit=500>
    /// - <https://api.bybit.com/v2/public/account-ratio?symbol=BTCUSDT&period=5min&limit=500>
    /// - <https://api.bybit.com/v2/public/account-ratio?symbol=BTCUSDU22&period=5min&limit=500>
    pub fn fetch_long_short_ratio(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/public/account-ratio?symbol={}&period=5min&limit=200",
            symbol
        ))
    }
}
