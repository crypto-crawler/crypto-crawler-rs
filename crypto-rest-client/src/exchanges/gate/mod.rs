mod gate_future;
mod gate_spot;
mod gate_swap;

pub use gate_future::GateFutureRestClient;
pub use gate_spot::GateSpotRestClient;
pub use gate_swap::GateSwapRestClient;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => gate_spot::GateSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap => {
            gate_swap::GateSwapRestClient::fetch_l2_snapshot
        }
        MarketType::LinearFuture => gate_future::GateFutureRestClient::fetch_l2_snapshot,
        _ => panic!("Gate unknown market_type: {}", market_type),
    };

    func(symbol)
}
