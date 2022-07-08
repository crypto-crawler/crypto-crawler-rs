use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, Precision, QuantityLimit,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    symbol: String,
    symbol_partition: String,
    price_precision: i64,
    min_order_amt: String,
    id: String,
    state: String,
    base_currency: String,
    amount_precision: i64,
    max_order_amt: Option<String>,
    quote_currency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct ResMsg {
    message: String,
    method: Option<String>,
    code: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    datas: Vec<SpotMarket>,
    resMsg: ResMsg,
}

// See https://zbgapi.github.io/docs/spot/v1/en/#public-get-all-supported-trading-symbols
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://www.zbg.com/exchange/api/v1/common/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.resMsg.code != "1" {
        Err(Error(txt))
    } else {
        let valid: Vec<SpotMarket> = resp
            .datas
            .into_iter()
            .filter(|x| x.state == "online")
            .collect();
        Ok(valid)
    }
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&m.symbol, "zbg").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "zbg".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol,
                base_id: m.base_currency,
                quote_id: m.quote_currency,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: m.state == "online",
                margin: false,
                // TODO: need to find zbg spot fees
                fees: Fees {
                    maker: 0.002,
                    taker: 0.002,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.price_precision as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.amount_precision as u32) as f64),
                },
                quantity_limit: Some(QuantityLimit {
                    min: m.min_order_amt.parse::<f64>().ok(),
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
