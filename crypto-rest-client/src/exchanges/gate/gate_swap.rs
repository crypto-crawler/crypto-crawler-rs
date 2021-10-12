use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.gateio.ws/api/v4";

/// The RESTful client for Gate Swap markets.
///
/// * RESTful API doc: <https://www.gate.io/docs/apiv4/en/index.html#futures>
/// * Trading at: <https://www.gateio.pro/cn/futures_trade/USDT/BTC_USDT>
/// * Rate Limits: <https://www.gate.io/docs/apiv4/en/index.html#frequency-limit-rule>
///   * 300 read operations per IP per second
pub struct GateSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl GateSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        GateSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 200 asks and bids are returned.
    ///
    /// For example:
    ///
    /// - <https://api.gateio.ws/api/v4/futures/btc/order_book?contract=BTC_USD&limit=200>
    /// - <https://api.gateio.ws/api/v4/futures/usdt/order_book?contract=BTC_USDT&limit=200>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let settle = if symbol.ends_with("_USD") {
            "btc"
        } else if symbol.ends_with("_USDT") {
            "usdt"
        } else {
            panic!("Unknown symbol {}", symbol);
        };
        gen_api!(format!(
            "/futures/{}/order_book?contract={}&limit=200",
            settle, symbol
        ))
    }

    /// Get open interest.
    ///
    /// For example:
    /// - <https://api.gateio.ws/api/v4/futures/btc/contract_stats?contract=BTC_USD&interval=5m>
    /// - <https://api.gateio.ws/api/v4/futures/usdt/contract_stats?contract=BTC_USDT&interval=5m>
    pub fn fetch_open_interest(symbol: &str) -> Result<String> {
        let settle = if symbol.ends_with("_USD") {
            "btc"
        } else if symbol.ends_with("_USDT") {
            "usdt"
        } else {
            panic!("Unknown symbol {}", symbol);
        };
        gen_api!(format!(
            "/futures/{}/contract_stats?contract={}&interval=5m",
            settle, symbol
        ))
    }
}
