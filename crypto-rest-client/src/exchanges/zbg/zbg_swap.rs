use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://www.zbg.com";

/// The RESTful client for ZBG swap markets.
///
/// * RESTful API doc: <https://zbgapi.github.io/docs/future/v1/en/>
/// * Trading at: <https://futures.zbg.com/>
pub struct ZbgSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl ZbgSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        ZbgSwapRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 200 bids and asks are returned.
    ///
    /// For example: <https://www.zbg.com/exchange/api/v1/future/market/depth?symbol=BTC_USD-R&size=200>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/exchange/api/v1/future/market/depth?symbol={symbol}&size=1000"))
    }

    /// Get open interest.
    ///
    /// For example:
    ///
    /// - <https://www.zbg.com/exchange/api/v1/future/market/ticker?symbol=BTC_USD-R>
    pub fn fetch_open_interest(symbol: &str) -> Result<String> {
        gen_api!(format!("/exchange/api/v1/future/market/ticker?symbol={symbol}"))
    }
}
