use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.hbdm.com";

/// Huobi Future market.
///
/// * REST API doc: <https://huobiapi.github.io/docs/dm/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/contract/exchange/>
/// * Rate Limits: <https://huobiapi.github.io/docs/dm/v1/en/#api-rate-limit-illustration>
///   * For restful interfaces：all products(futures, coin margined swap, usdt margined swap ) 800 times/second for one IP at most
pub struct HuobiFutureRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiFutureRestClient);

impl HuobiFutureRestClient {
    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 150 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.hbdm.com/market/depth?symbol=BTC_CQ&type=step0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/market/depth?symbol={}&type=step0", symbol))
    }

    /// Get open interest.
    ///
    /// For example: <https://api.hbdm.com/api/v1/contract_open_interest?contract_code=BTC211231>
    pub fn fetch_open_interest(symbol: Option<&str>) -> Result<String> {
        if let Some(symbol) = symbol {
            gen_api!(format!(
                "/api/v1/contract_open_interest?contract_code={}",
                symbol
            ))
        } else {
            gen_api!("/api/v1/contract_open_interest")
        }
    }
}
