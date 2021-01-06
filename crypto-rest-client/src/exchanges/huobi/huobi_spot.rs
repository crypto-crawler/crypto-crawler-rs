use super::super::utils::http_get;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.huobi.pro";

/// Huobi Spot market.
///
/// * REST API doc: <https://huobiapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.huobi.com/en-us/exchange/>
pub struct HuobiSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiSpotRestClient);
