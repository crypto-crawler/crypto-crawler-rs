use super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://www.bitstamp.net/api";

/// The REST client for Bitstamp.
///
/// Bitstamp has only Spot market.
///
///   * REST API doc: <https://www.bitstamp.net/api/>
///   * Trading at: <https://www.bitstamp.net/market/tradeview/>
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
    pub fn fetch_trades(symbol: &str, time: Option<String>) -> Result<String, reqwest::Error> {
        gen_api!(format!("/v2/transactions/{}/", symbol), time)
    }

    /// Get a full L2 orderbook snapshot.
    ///
    /// /// Equivalent to `/order_book/symbol` with `group=1`
    ///
    /// For example: <https://www.bitstamp.net/api/v2/order_book/btcusd/>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        gen_api!(format!("/order_book/{}", symbol))
    }

    /// Get a full L3 orderbook snapshot.
    ///
    /// Equivalent to `/order_book/symbol` with `group=2`
    ///
    /// For example: <https://www.bitstamp.net/api/v2/order_book/btcusd/?group=2>
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String, reqwest::Error> {
        gen_api!(format!("/order_book/{}?group=2", symbol))
    }
}
