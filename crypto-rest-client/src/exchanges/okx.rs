use super::utils::http_get;
use crate::error::Result;
use crypto_market_type::MarketType;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

const BASE_URL: &str = "https://www.okx.com";

/// The REST client for OKEx.
///
/// OKEx has Spot, Future, Swap and Option markets.
///
/// * API doc: <https://www.okx.com/docs-v5/en/>
/// * Trading at:
///     * Spot <https://www.okx.com/trade-spot>
///     * Future <https://www.okx.com/trade-futures>
///     * Swap <https://www.okx.com/trade-swap>
///     * Option <https://www.okx.com/trade-option>
pub struct OkxRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl OkxRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        OkxRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get most recent trades.
    ///
    /// 500 trades are returned.
    ///
    /// For example: <https://www.okx.com/api/v5/market/trades?instId=BTC-USDT&limit=500>
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v5/market/trades?instId={}&limit=500", symbol))
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 400 bids and asks are returned.
    ///
    /// For example:
    /// * <https://www.okx.com/api/v5/market/books?instId=BTC-USDT&sz=400>,
    /// * <https://www.okx.com/api/v5/market/books?instId=BTC-USDT-SWAP&sz=400>
    ///
    /// Rate limit: 20 requests per 2 seconds
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v5/market/books?instId={}&sz=400", symbol,))
    }

    /// Get option underlying.
    pub fn fetch_option_underlying() -> Result<Vec<String>> {
        let txt = http_get(
            "https://www.okx.com/api/v5/public/underlying?instType=OPTION",
            &BTreeMap::new(),
        )?;
        let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
        let data = json_obj.get("data").unwrap().as_array().unwrap()[0].as_array().unwrap();
        let underlying_indexes =
            data.iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<String>>();
        Ok(underlying_indexes)
    }

    /// Get open interest.
    ///
    /// inst_type: SWAP, FUTURES, OPTION
    ///
    /// For example:
    /// - <https://www.okx.com/api/v5/public/open-interest?instType=SWAP>
    /// - <https://www.okx.com/api/v5/public/open-interest?instType=SWAP&instId=BTC-USD-SWAP>
    pub fn fetch_open_interest(market_type: MarketType, symbol: Option<&str>) -> Result<String> {
        let inst_type = match market_type {
            MarketType::LinearFuture => "FUTURES",
            MarketType::InverseFuture => "FUTURES",
            MarketType::LinearSwap => "SWAP",
            MarketType::InverseSwap => "SWAP",
            MarketType::EuropeanOption => "OPTION",
            _ => panic!("okx {} doesn't have open interest", market_type),
        };
        if let Some(inst_id) = symbol {
            gen_api!(format!(
                "/api/v5/public/open-interest?instType={}&instId={}",
                inst_type, inst_id,
            ))
        } else {
            gen_api!(format!("/api/v5/public/open-interest?instType={}", inst_type))
        }
    }
}
