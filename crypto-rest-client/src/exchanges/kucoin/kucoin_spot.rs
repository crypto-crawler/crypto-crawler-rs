use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.kucoin.com";

/// The RESTful client for KuCoin spot market.
///
/// * RESTful API doc: <https://docs.kucoin.com/>
/// * Trading at: <https://trade.kucoin.com/>
/// * Rate Limits: <https://docs.kucoin.com/#request-rate-limit>
pub struct KuCoinSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl KuCoinSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        KuCoinSpotRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// For example: <https://api.kucoin.com/api/v1/market/orderbook/level2_100?symbol=BTC-USDT>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let api = if std::env::var("KC-API-KEY").is_ok() {
            // the request rate limit is 30 times/3s
            "/api/v3/market/orderbook/level2"
        } else {
            "/api/v1/market/orderbook/level2_100"
        };
        gen_api!(format!("{}?symbol={}", api, symbol))
    }

    /// Get the latest Level3 snapshot of orderbook.
    ///
    /// All bids and asks are returned.
    ///
    /// For example: <https://api.kucoin.com/api/v2/market/orderbook/level3?symbol=BTC-USDT>,
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v2/market/orderbook/level3?symbol={}", symbol))
    }
}
