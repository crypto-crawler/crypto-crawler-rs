use super::utils::http_get;
use crate::{error::Result, Market, MarketType};

use serde::{Deserialize, Serialize};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    id: String,
    base_currency: String,
    quote_currency: String,
    base_min_size: String,
    base_max_size: String,
    quote_increment: String,
    base_increment: String,
    display_name: String,
    min_market_funds: String,
    max_market_funds: String,
    margin_enabled: bool,
    post_only: bool,
    limit_only: bool,
    cancel_only: bool,
    trading_disabled: bool,
    status: String,
    status_message: String,
}

// see <https://docs.pro.coinbase.com/#products>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.pro.coinbase.com/products", None)?;
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
