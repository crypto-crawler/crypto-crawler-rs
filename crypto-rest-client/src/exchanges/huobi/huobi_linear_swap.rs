use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.hbdm.com";

/// Huobi Linear Swap market.
///
/// Linear Swap market uses USDT as collateral.
///
/// * REST API doc: <https://huobiapi.github.io/docs/usdt_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/linear_swap/exchange/>
/// * Rate Limits: <https://huobiapi.github.io/docs/usdt_swap/v1/en/#api-rate-limit-illustration>
///   * For restful interfaces, products, (future, coin margined swap, usdt margined swap)800 times/second for one IP at most
pub struct HuobiLinearSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiLinearSwapRestClient);

impl HuobiLinearSwapRestClient {
    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 150 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.hbdm.com/linear-swap-ex/market/depth?contract_code=BTC-USDT&type=step0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/linear-swap-ex/market/depth?contract_code={}&type=step0",
            symbol
        ))
    }

    /// Get open interest.
    ///
    /// For example: <https://api.hbdm.com/linear-swap-api/v1/swap_open_interest?contract_code=BTC-USDT>
    pub fn fetch_open_interest(symbol: Option<&str>) -> Result<String> {
        if let Some(symbol) = symbol {
            gen_api!(format!(
                "/linear-swap-api/v1/swap_open_interest?contract_code={}",
                symbol
            ))
        } else {
            gen_api!("/linear-swap-api/v1/swap_open_interest")
        }
    }
}
