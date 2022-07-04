use super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.exchange.coinbase.com";

/// The REST client for CoinbasePro.
///
/// CoinbasePro has only Spot market.
///
///   * REST API doc: <https://docs.cloud.coinbase.com/exchange/reference>
///   * Trading at: <https://pro.coinbase.com/>
///   * Rate Limits: <https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaccounts#rate-limits>
///   * This endpoint has a custom rate limit by profile ID: 25 requests per second, up to 50 requests per second in bursts
pub struct CoinbaseProRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl CoinbaseProRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        CoinbaseProRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// List the latest trades for a product.
    ///
    /// `/products/{symbol}/trades`
    ///
    /// For example: <https://api.pro.coinbase.com/products/BTC-USD/trades>
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        gen_api!(format!("/products/{}/trades", symbol))
    }

    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 50 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.pro.coinbase.com/products/BTC-USD/book?level=2>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/products/{}/book?level=2", symbol))
    }

    /// Get the latest Level3 orderbook snapshot.
    ///
    /// Full order book (non aggregated) are returned.
    ///
    /// For example: <https://api.pro.coinbase.com/products/BTC-USD/book?level=3>
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/products/{}/book?level=3", symbol))
    }
}
