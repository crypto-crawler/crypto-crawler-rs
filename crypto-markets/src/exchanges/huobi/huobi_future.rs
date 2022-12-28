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

// https://github.com/ccxt/ccxt/issues/8074

#[derive(Serialize, Deserialize)]
struct FutureMarket {
    symbol: String,
    contract_code: String,
    contract_type: String,
    contract_size: f64,
    price_tick: f64,
    delivery_date: String,
    delivery_time: String,
    create_date: String,
    contract_status: i64,
    settlement_time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<FutureMarket>,
    ts: i64,
}

// see <https://huobiapi.github.io/docs/dm/v1/en/#get-contract-info>
fn fetch_future_markets_raw() -> Result<Vec<FutureMarket>> {
    let txt = huobi_http_get("https://api.hbdm.com/api/v1/contract_contract_info")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    let result: Vec<FutureMarket> =
        resp.data.into_iter().filter(|m| m.contract_status == 1).collect();
    Ok(result)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .map(|m| {
            m.symbol.to_string()
                + match m.contract_type.as_str() {
                    "this_week" => "_CW",
                    "next_week" => "_NW",
                    "quarter" => "_CQ",
                    "next_quarter" => "_NQ",
                    contract_type => panic!("Unknown contract_type {contract_type}"),
                }
        })
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_raw()?
        .into_iter()
        .map(|m| {
            let symbol = m.symbol.to_string()
                + match m.contract_type.as_str() {
                    "this_week" => "_CW",
                    "next_week" => "_NW",
                    "quarter" => "_CQ",
                    "next_quarter" => "_NQ",
                    contract_type => panic!("Unknown contract_type {contract_type}"),
                };
            let pair = crypto_pair::normalize_pair(&symbol, "huobi").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "huobi".to_string(),
                market_type: MarketType::InverseFuture,
                symbol,
                base_id: m.symbol.to_string(),
                quote_id: "USD".to_string(),
                settle_id: Some(m.symbol.to_string()),
                base: base.clone(),
                quote,
                settle: Some(base),
                active: m.contract_status == 1,
                margin: true,
                // see https://futures.huobi.com/en-us/contract/fee_rate/
                fees: Fees { maker: 0.0002, taker: 0.0004 },
                precision: Precision { tick_size: m.price_tick, lot_size: 1.0 },
                quantity_limit: None,
                contract_value: Some(m.contract_size),
                delivery_date: Some(m.delivery_time.parse::<u64>().unwrap()),
                info: serde_json::to_value(&m).unwrap().as_object().unwrap().clone(),
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
