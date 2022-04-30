mod bitget_spot;
mod bitget_swap;
mod utils;

pub use bitget_spot::BitgetSpotWSClient;
pub use bitget_swap::BitgetSwapWSClient;

pub(super) const EXCHANGE_NAME: &str = "bitget";
