use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{error::Result, Fees, Market, Precision, QuantityLimit};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// https://www.gateio.pro/docs/apiv4/zh_CN/#currencypair
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    id: String,
    base: String,
    quote: String,
    fee: String,
    min_base_amount: Option<String>,
    min_quote_amount: Option<String>,
    amount_precision: i64,
    precision: i64,
    trade_status: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#611e43ef81
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.gateio.ws/api/v4/spot/currency_pairs", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt)?;
    Ok(markets
        .into_iter()
        .filter(|x| x.trade_status == "tradable")
        .collect::<Vec<SpotMarket>>())
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.id).collect();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_spot_markets_raw()?
        .into_iter()
        .map(|raw_market| {
            let info = serde_json::to_value(&raw_market)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&raw_market.id, "gate").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };

            Market {
                exchange: "gate".to_string(),
                market_type: MarketType::Spot,
                symbol: raw_market.id.to_string(),
                base_id: raw_market.base,
                settle_id: None,
                quote_id: raw_market.quote,
                base,
                quote,
                settle: None,
                active: raw_market.trade_status == "tradable",
                margin: false,
                fees: Fees {
                    maker: raw_market.fee.parse::<f64>().unwrap() / 100_f64,
                    taker: raw_market.fee.parse::<f64>().unwrap() / 100_f64,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(raw_market.precision as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(raw_market.amount_precision as u32) as f64),
                },
                quantity_limit: raw_market
                    .min_base_amount
                    .map(|min_base_amount| QuantityLimit {
                        min: min_base_amount.parse::<f64>().ok(),
                        max: None,
                        notional_min: None,
                        notional_max: None,
                    }),
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect();
    Ok(markets)
}
