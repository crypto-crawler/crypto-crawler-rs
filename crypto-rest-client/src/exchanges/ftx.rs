use super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://ftx.com/api";

/// The RESTful client for FTX.
///
/// FTX has Spot, LinearFuture, LinearSwap, Option, Move and BVOL markets.
///
/// * RESTful API doc: <https://docs.ftx.com/#rest-api>
/// * Trading at <https://ftx.com/markets>
/// * Rate Limits: <https://docs.ftx.com/?python#rate-limits>
///   * Non-order placement requests do not count towards rate limits.
///   * Rate limits are tiered by account trading volumes.
pub struct FtxRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl FtxRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        FtxRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 100 bids and asks are returned.
    ///
    /// For example: <https://ftx.com/api/markets/BTC-PERP/orderbook?depth=100>,
    // <https://ftx.com/api/markets/BTC/USD/orderbook?depth=100>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/markets/{}/orderbook?depth=100", symbol))
    }

    /// Get open interest.
    ///
    /// For example:
    /// - <https://ftx.com/api/futures>
    pub fn fetch_open_interest() -> Result<String> {
        gen_api!("/futures")
    }
}
