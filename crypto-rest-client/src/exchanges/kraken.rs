use super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.kraken.com";

/// The WebSocket client for Kraken.
///
/// Kraken has only Spot market.
///
/// * REST API doc: <https://docs.kraken.com/rest/>
/// * Trading at: <https://trade.kraken.com/>
/// * Rate Limits: <https://docs.kraken.com/rest/#section/Rate-Limits/REST-API-Rate-Limits>
///   * 15 requests per 45 seconds
pub struct KrakenRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl KrakenRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        KrakenRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get most recent trades.
    ///
    /// If `since` is provided, return trade data since given id (exclusive).
    ///
    /// For example: <https://api.kraken.com/0/public/Trades?pair=XXBTZUSD&since=1609893937598797338>
    #[allow(non_snake_case)]
    pub fn fetch_trades(symbol: &str, since: Option<String>) -> Result<String> {
        gen_api!(format!("/0/public/Trades?pair={}", symbol), since)
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Top 500 bids and asks are returned.
    ///
    /// For example: <https://api.kraken.com/0/public/Depth?pair=XXBTZUSD&count=500>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/0/public/Depth?pair={}&count=500", symbol))
    }
}
