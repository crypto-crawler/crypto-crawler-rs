use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.hbdm.com";

/// Huobi Inverse Swap market.
///
/// Inverse Swap market uses coins like BTC as collateral.
///
/// * REST API doc: <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/swap/exchange/>
/// * Rate Limits: <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#api-rate-limit-illustration>
///  * For restful interfaces：all products(futures, coin margined swap, usdt
///    margined swap) 800 times/second for one IP at most
pub struct HuobiInverseSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiInverseSwapRestClient);

impl HuobiInverseSwapRestClient {
    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 150 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.hbdm.com/swap-ex/market/depth?contract_code=BTC-USD&type=step0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/swap-ex/market/depth?contract_code={symbol}&type=step0"))
    }

    /// Get open interest.
    ///
    /// For example: <https://api.hbdm.com/swap-api/v1/swap_open_interest?contract_code=BTC-USD>
    pub fn fetch_open_interest(symbol: Option<&str>) -> Result<String> {
        if let Some(symbol) = symbol {
            gen_api!(format!("/swap-api/v1/swap_open_interest?contract_code={symbol}"))
        } else {
            gen_api!("/swap-api/v1/swap_open_interest")
        }
    }
}
