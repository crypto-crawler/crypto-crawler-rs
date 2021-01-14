use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(_market_type: MarketType) -> Result<Vec<String>> {
    Ok(Vec::new())
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}
