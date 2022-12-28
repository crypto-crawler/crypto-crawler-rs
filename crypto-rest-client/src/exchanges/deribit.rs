use super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://www.deribit.com/api/v2";

/// The RESTful client for Deribit.
///
/// Deribit has InverseFuture, InverseSwap and Option markets.
///
/// * WebSocket API doc: <https://docs.deribit.com/?shell#market-data>
/// * Trading at:
///     * Future <https://www.deribit.com/main#/futures>
///     * Option <https://www.deribit.com/main#/options>
/// * Rate Limits: <https://www.deribit.com/pages/information/rate-limits>
///   * Each sub-account has a rate limit of 20 requests per second
pub struct DeribitRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl DeribitRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        DeribitRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get most recent trades.
    ///
    /// 100 trades are returned.
    ///
    /// For example: <https://www.deribit.com/api/v2/public/get_last_trades_by_instrument?count=100&instrument_name=BTC-PERPETUAL>
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/public/get_last_trades_by_instrument?count=100&instrument_name={}",
            symbol
        ))
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 2000 bids and asks are returned.
    ///
    /// For example: <https://www.deribit.com/api/v2/public/get_order_book?depth=2000&instrument_name=BTC-PERPETUAL>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/public/get_order_book?depth=2000&instrument_name={}", symbol,))
    }

    /// Get open interest.
    ///
    /// For example:
    /// - <https://www.deribit.com/api/v2/public/get_book_summary_by_currency?currency=BTC>
    /// - <https://www.deribit.com/api/v2/public/get_book_summary_by_instrument?instrument_name=BTC-PERPETUAL>
    pub fn fetch_open_interest(symbol: Option<&str>) -> Result<String> {
        if let Some(symbol) = symbol {
            gen_api!(format!("/public/get_book_summary_by_instrument?instrument_name={}", symbol))
        } else {
            let btc = gen_api!("/public/get_book_summary_by_currency?currency=BTC")?;
            let eth = gen_api!("/public/get_book_summary_by_currency?currency=ETH")?;
            let sol = gen_api!("/public/get_book_summary_by_currency?currency=SOL")?;
            let usdc = gen_api!("/public/get_book_summary_by_currency?currency=USDC")?;
            Ok(format!("{}\n{}\n{}\n{}", btc, eth, sol, usdc))
        }
    }
}
