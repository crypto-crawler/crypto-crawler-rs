use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com/option-ex";

/// Huobi Option market.
///
///
/// * REST API doc: <https://huobiapi.github.io/docs/option/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/option/exchange/>
pub struct HuobiOptionRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiOptionRestClient);

impl HuobiOptionRestClient {
    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 150 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.hbdm.com/option-ex/market/depth?contract_code=BTC-USDT-210326-C-32000&type=step0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/market/depth?contract_code={}&type=step0", symbol))
    }
}
