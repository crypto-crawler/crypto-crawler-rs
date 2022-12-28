use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.bitget.com";

/// The RESTful client for Bitget spot market.
///
/// * RESTful API doc: <https://bitgetlimited.github.io/apidoc/en/spot/>
/// * Trading at: <https://www.bitget.com/spot/>
pub struct BitgetSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitgetSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitgetSpotRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 150 bids and asks are returned.
    ///
    /// For example: <https://api.bitget.com/api/spot/v1/market/depth?symbol=BTCUSDT_SPBL&type=step0>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/spot/v1/market/depth?symbol={symbol}&type=step0"))
    }
}
