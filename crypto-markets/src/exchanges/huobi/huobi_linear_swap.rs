use super::utils::huobi_http_get;
use crate::{
    error::Result,
    market::{Fees, Precision},
    Market,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct LinearSwapMarket {
    symbol: String,
    contract_code: String,
    contract_size: f64,
    price_tick: f64,
    delivery_time: String,
    create_date: String,
    contract_status: i64,
    settlement_date: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<LinearSwapMarket>,
    ts: i64,
}

// see <https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-query-swap-info>
fn fetch_linear_swap_markets_raw() -> Result<Vec<LinearSwapMarket>> {
    let txt = huobi_http_get("https://api.hbdm.com/linear-swap-api/v1/swap_contract_info")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    let result: Vec<LinearSwapMarket> =
        resp.data.into_iter().filter(|m| m.contract_status == 1).collect();
    Ok(result)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_linear_swap_markets_raw()?
        .into_iter()
        .map(|m| m.contract_code)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_linear_swap_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m).unwrap().as_object().unwrap().clone();
            let pair = crypto_pair::normalize_pair(&m.contract_code, "huobi").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "huobi".to_string(),
                market_type: MarketType::LinearSwap,
                symbol: m.contract_code,
                base_id: m.symbol.to_string(),
                quote_id: "USDT".to_string(),
                settle_id: Some("USDT".to_string()),
                base,
                quote: quote.clone(),
                settle: Some(quote),
                active: m.contract_status == 1,
                margin: true,
                // see https://futures.huobi.com/en-us/linear_swap/fee_rate/
                fees: Fees { maker: 0.0002, taker: 0.0004 },
                precision: Precision { tick_size: m.price_tick, lot_size: 1.0 },
                quantity_limit: None,
                contract_value: Some(m.contract_size),
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
