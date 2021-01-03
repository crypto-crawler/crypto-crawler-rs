mod ws_stream;
#[macro_use]
mod ws_client_internal;

mod utils;

pub(super) mod binance;
pub(super) mod bitfinex;
pub(super) mod bitmex;
pub(super) mod bitstamp;
pub(super) mod coinbase_pro;
pub(super) mod huobi;
pub(super) mod kraken;
pub(super) mod mxc;
pub(super) mod okex;

// tick-by-tick trade
trait Trade {
    fn subscribe_trade(&mut self, pairs: &[String]);
}

// 24hr rolling window ticker
trait Ticker {
    fn subscribe_ticker(&mut self, pairs: &[String]);
}

// Best Bid & Offer
pub trait BBO {
    fn subscribe_bbo(&mut self, pairs: &[String]);
}
