use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.zb.com";

/// The RESTful client for ZB spot market.
///
/// * RESTful API doc: <https://www.zb.com/en/api>
/// * Trading at: <https://www.zb.com/en/kline/btc_usdt>
pub struct ZbSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl ZbSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        ZbSpotRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 50 bids and asks are returned.
    ///
    /// For example: <https://api.zbex.site/data/v1/depth?market=btc_usdt&size=50>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/data/v1/depth?market={symbol}&size=50"))
    }
}
