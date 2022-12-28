use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://fapi.zb.com";

/// The RESTful client for ZB swap markets.
///
/// * RESTful API doc: <https://www.zb.com/en/contract-api>
/// * Trading at: <https://www.zb.com/en/futures/btc_usdt>
pub struct ZbSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl ZbSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        ZbSwapRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 200 bids and asks are returned.
    ///
    /// For example:
    /// * <https://fapi.zb.com/api/public/v1/depth?symbol=BTC_USDT&size=200>
    /// * <https://fapi.zb.com/qc/api/public/v1/depth?symbol=BTC_QC&size=200>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        if symbol.ends_with("_QC") {
            gen_api!(format!("/qc/api/public/v1/depth?symbol={}&size=200", symbol))
        } else {
            gen_api!(format!("/api/public/v1/depth?symbol={}&size=200", symbol))
        }
    }
}
