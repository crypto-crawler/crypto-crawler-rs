mod kraken_futures;
mod kraken_spot;

const EXCHANGE_NAME: &str = "kraken";

pub use kraken_futures::KrakenFuturesWSClient;
pub use kraken_spot::KrakenSpotWSClient;
