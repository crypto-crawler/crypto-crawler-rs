use super::utils::huobi_http_get;
use crate::{
    error::Result,
    market::{Fees, Precision, QuantityLimit},
    Market,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    price_precision: f64,
    amount_precision: f64,
    symbol_partition: String,
    symbol: String,
    state: String,
    value_precision: f64,
    min_order_amt: f64,
    max_order_amt: f64,
    min_order_value: f64,
    limit_order_min_order_amt: f64,
    limit_order_max_order_amt: f64,
    sell_market_min_order_amt: f64,
    sell_market_max_order_amt: f64,
    buy_market_max_order_value: f64,
    api_trading: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<SpotMarket>,
}

// see <https://huobiapi.github.io/docs/spot/v1/en/#get-all-supported-trading-symbol>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = huobi_http_get("https://api.huobi.pro/v1/common/symbols")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    let result: Vec<SpotMarket> = resp
        .data
        .into_iter()
        .filter(|m| m.state == "online")
        .collect();
    Ok(result)
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
            let pair = crypto_pair::normalize_pair(&m.symbol, "huobi").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "huobi".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol,
                base_id: m.base_currency.to_string(),
                quote_id: m.quote_currency.to_string(),
                settle_id: None,
                base,
                quote,
                settle: None,
                active: m.state == "online",
                margin: true,
                // see https://www.huobi.com/en-us/fee/
                fees: Fees {
                    maker: 0.002,
                    taker: 0.002,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.price_precision as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.amount_precision as u32) as f64),
                },
                quantity_limit: Some(QuantityLimit {
                    min: Some(m.limit_order_min_order_amt),
                    max: Some(m.limit_order_max_order_amt),
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
