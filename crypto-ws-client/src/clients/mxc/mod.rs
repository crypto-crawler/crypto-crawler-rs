mod mxc_spot;
mod mxc_swap;

pub(super) const EXCHANGE_NAME: &str = "mxc";

pub use mxc_spot::MxcSpotWSClient;
pub use mxc_swap::MxcSwapWSClient;
