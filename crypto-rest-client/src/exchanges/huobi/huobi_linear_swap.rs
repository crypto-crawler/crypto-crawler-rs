use super::super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com/linear-swap-ex";

/// Huobi Linear Swap market.
///
/// Linear Swap market uses USDT as collateral.
///
/// * REST API doc: <https://huobiapi.github.io/docs/usdt_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/linear_swap/exchange/>
pub struct HuobiLinearSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiLinearSwapRestClient);
