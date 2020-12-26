use crate::{Market, MarketType};

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>, reqwest::Error> {
    Ok(Vec::new())
}
