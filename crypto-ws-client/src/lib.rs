mod clients;
mod common;

pub use common::ws_client::WSClient;

pub use clients::{
    binance::*, binance_option::*, bitfinex::*, bitget::*, bithumb::*, bitmex::*, bitstamp::*,
    bitz::*, bybit::*, coinbase_pro::*, deribit::*, dydx::*, ftx::*, gate::*, huobi::*, kraken::*,
    kucoin::*, mexc::*, okx::*, zbg::*,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
