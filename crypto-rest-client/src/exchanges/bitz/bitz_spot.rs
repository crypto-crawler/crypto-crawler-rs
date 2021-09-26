use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://apiv2.bitz.com";

/// The RESTful client for BitZ spot market.
///
/// * RESTful API doc: <https://apidocv2.bitz.plus/en/>
/// * Trading at: <https://www.bitz.plus/exchange>
/// * Rate Limits: <https://apidocv2.bitz.plus/en/#limit>
///   * no more than 30 times within 1 sec
pub struct BitzSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitzSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitzSpotRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// For example: <https://apiv2.bitz.com/V2/Market/depth?symbol=btc_usdt>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/V2/Market/depth?symbol={}", symbol))
    }
}
