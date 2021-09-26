use super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://global-openapi.bithumb.pro/openapi/v1";

/// The REST client for Bithumb.
///
/// Bithumb has only Spot market.
///
/// * REST API doc: <https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/rest-api.md>
/// * Trading at: <https://en.bithumb.com/trade/order/BTC_KRW>
/// * Rate Limits: <https://apidocs.bithumb.com/docs/rate_limits>
///   * 135 requests per 1 second for public APIs.
///   * 15 requests per 1 second for private APIs.
pub struct BithumbRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BithumbRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BithumbRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get most recent trades.
    ///
    /// For example: <https://global-openapi.bithumb.pro/openapi/v1/spot/trades?symbol=BTC-USDT>
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        gen_api!(format!("/spot/trades?symbol={}", symbol))
    }

    /// Get the latest Level2 orderbook snapshot.
    ///
    /// For example: <https://global-openapi.bithumb.pro/openapi/v1/spot/orderBook?symbol=BTC-USDT>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/spot/orderBook?symbol={}", symbol))
    }
}
