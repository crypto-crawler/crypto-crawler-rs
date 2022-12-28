use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://kline.zbg.com";

/// The RESTful client for ZBG spot market.
///
/// * RESTful API doc: <https://zbgapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.zbg.com/trade/>
pub struct ZbgSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl ZbgSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        ZbgSpotRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 200 bids and asks are returned.
    ///
    /// For example: <https://kline.zbg.com/api/data/v1/entrusts?marketName=btc_usdt&dataSize=200>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/data/v1/entrusts?marketName={}&dataSize=200", symbol))
    }
}
