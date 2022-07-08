use std::collections::HashMap;

use super::utils::http_get;
use crate::{error::Result, Fees, Market, MarketType, Precision, QuantityLimit};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    id: String,
    base_currency: String,
    quote_currency: String,
    quote_increment: String,
    base_increment: String,
    display_name: String,
    min_market_funds: Option<String>,
    max_market_funds: Option<String>,
    margin_enabled: bool,
    post_only: bool,
    limit_only: bool,
    cancel_only: bool,
    trading_disabled: bool,
    status: String,
    status_message: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getproducts>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.exchange.coinbase.com/products", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt)?;
    Ok(markets)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter(|m| !m.trading_disabled && m.status == "online" && !m.cancel_only)
        .map(|m| m.id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&m.id, "coinbase_pro").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "coinbase_pro".to_string(),
                market_type: MarketType::Spot,
                symbol: m.id,
                base_id: m.base_currency,
                quote_id: m.quote_currency,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: !m.trading_disabled && m.status == "online" && !m.cancel_only,
                margin: m.margin_enabled,
                // // see https://pro.coinbase.com/fees, https://pro.coinbase.com/orders/fees
                fees: Fees {
                    maker: 0.005,
                    taker: 0.005,
                },
                precision: Precision {
                    tick_size: m.quote_increment.parse::<f64>().unwrap(),
                    lot_size: m.base_increment.parse::<f64>().unwrap(),
                },
                quantity_limit: Some(QuantityLimit {
                    min: None,
                    max: None,
                    notional_min: m.min_market_funds.map(|x| x.parse::<f64>().unwrap()),
                    notional_max: m.max_market_funds.map(|x| x.parse::<f64>().unwrap()),
                }),
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
