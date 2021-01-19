use super::utils::http_get;
use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}

// see <https://docs.bitfinex.com/reference#rest-public-conf>
fn fetch_spot_symbols() -> Result<Vec<String>> {
    let text = http_get(
        "https://api-pub.bitfinex.com/v2/conf/pub:list:pair:exchange",
        None,
    )?;
    let pairs = serde_json::from_str::<Vec<Vec<String>>>(&text)?;
    let symbols = pairs[0]
        .iter()
        .filter(|x| !x.starts_with("TEST"))
        .map(|p| format!("t{}", p))
        .collect::<Vec<String>>();
    Ok(symbols)
}

// see <https://docs.bitfinex.com/reference#rest-public-conf>
fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let text = http_get(
        "https://api-pub.bitfinex.com/v2/conf/pub:list:pair:futures",
        None,
    )?;
    let pairs = serde_json::from_str::<Vec<Vec<String>>>(&text)?;
    let symbols = pairs[0]
        .iter()
        .filter(|x| !x.starts_with("TEST"))
        .map(|p| format!("t{}", p))
        .collect::<Vec<String>>();
    Ok(symbols)
}
