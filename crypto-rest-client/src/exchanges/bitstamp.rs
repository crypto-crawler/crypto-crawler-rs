use super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://www.bitstamp.net/api";

/// The REST client for Bitstamp.
///
/// Bitstamp has only Spot market.
///
/// * REST API doc: <https://www.bitstamp.net/api/>
/// * Trading at: <https://www.bitstamp.net/market/tradeview/>
/// * Rate Limits: <https://www.bitstamp.net/api/#what-is-api>
///   * Do not make more than 8000 requests per 10 minutes or we will ban your IP address.
pub struct BitstampRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitstampRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitstampRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get trades.
    ///
    /// `/v2/transactions/{symbol}/`
    ///
    /// `time` specifies the time interval from which we want the transactions
    /// to be returned. Possible values are "minute", "hour" (default) or "day".
    ///
    /// For example: <https://www.bitstamp.net/api/v2/transactions/btcusd/?time=hour>
    pub fn fetch_trades(symbol: &str, time: Option<String>) -> Result<String> {
        gen_api!(format!("/v2/transactions/{}/", symbol), time)
    }

    /// Get a full Level2 orderbook snapshot.
    ///
    /// /// Equivalent to `/order_book/symbol` with `group=1`
    ///
    /// For example: <https://www.bitstamp.net/api/v2/order_book/btcusd/>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/order_book/{}", symbol))
    }

    /// Get a full Level3 orderbook snapshot.
    ///
    /// Equivalent to `/order_book/symbol` with `group=2`
    ///
    /// For example: <https://www.bitstamp.net/api/v2/order_book/btcusd/?group=2>
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/order_book/{}?group=2", symbol))
    }
}
