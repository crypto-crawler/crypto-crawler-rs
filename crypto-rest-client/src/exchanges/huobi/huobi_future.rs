use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com";

/// Huobi Future market.
///
/// * REST API doc: <https://huobiapi.github.io/docs/dm/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/contract/exchange/>
pub struct HuobiFutureRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiFutureRestClient);
