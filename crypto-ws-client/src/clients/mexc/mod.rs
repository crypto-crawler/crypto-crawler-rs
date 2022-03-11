mod mexc_spot;
mod mexc_swap;

pub(super) const EXCHANGE_NAME: &str = "mexc";

pub use mexc_spot::MexcSpotWSClient;
pub use mexc_swap::MexcSwapWSClient;
