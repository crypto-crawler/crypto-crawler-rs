pub(crate) mod kraken_futures;
pub(crate) mod kraken_spot;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => kraken_spot::KrakenSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseFuture | MarketType::InverseSwap => {
            kraken_futures::KrakenFuturesRestClient::fetch_l2_snapshot
        }
        _ => panic!("Kraken unknown market_type: {}", market_type),
    };

    func(symbol)
}
