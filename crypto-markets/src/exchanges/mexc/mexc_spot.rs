use super::utils::mexc_http_get;
use crate::{error::Result, Fees, Market, Precision, QuantityLimit};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    symbol: String,
    state: String,
    price_scale: u32,
    quantity_scale: u32,
    min_amount: String,
    max_amount: String,
    maker_fee_rate: String,
    taker_fee_rate: String,
    limited: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: i64,
    data: Vec<SpotMarket>,
}

// see <https://mxcdevelop.github.io/APIDoc/open.api.v2.en.html#all-symbols>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = mexc_http_get("https://www.mexc.com/open/api/v2/market/symbols")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    Ok(resp
        .data
        .into_iter()
        .filter(|m| m.state == "ENABLED" && !m.limited)
        .collect())
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&m.symbol, super::EXCHANGE_NAME).unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            let (base_id, quote_id) = {
                let v: Vec<&str> = m.symbol.split('_').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: super::EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol,
                base_id,
                quote_id,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: m.state == "ENABLED" && !m.limited,
                margin: false,
                fees: Fees {
                    maker: m.maker_fee_rate.parse::<f64>().unwrap(),
                    taker: m.taker_fee_rate.parse::<f64>().unwrap(),
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.price_scale) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.quantity_scale) as f64),
                },
                quantity_limit: Some(QuantityLimit {
                    min: m.min_amount.parse::<f64>().ok(),
                    max: Some(m.max_amount.parse::<f64>().unwrap()),
                    notional_min: None,
                    notional_max: None,
                }),
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
