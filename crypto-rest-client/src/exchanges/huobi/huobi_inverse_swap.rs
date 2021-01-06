use super::super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com/swap-ex";

/// Huobi Inverse Swap market.
///
/// Inverse Swap market uses coins like BTC as collateral.
///
/// * REST API doc: <<https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>>
/// * Trading at: <https://futures.huobi.com/en-us/swap/exchange/>
pub struct HuobiInverseSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiInverseSwapRestClient);
