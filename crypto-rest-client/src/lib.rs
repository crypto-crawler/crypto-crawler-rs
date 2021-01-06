mod exchanges;

pub use exchanges::binance::binance_future::BinanceFutureRestClient;
pub use exchanges::binance::binance_inverse_swap::BinanceInverseSwapRestClient;
pub use exchanges::binance::binance_linear_swap::BinanceLinearSwapRestClient;
pub use exchanges::binance::binance_spot::BinanceSpotRestClient;
pub use exchanges::bitfinex::BitfinexRestClient;
pub use exchanges::bitmex::BitMEXRestClient;
pub use exchanges::bitstamp::BitstampRestClient;
pub use exchanges::coinbase_pro::CoinbaseProRestClient;
pub use exchanges::huobi::huobi_future::HuobiFutureRestClient;
pub use exchanges::huobi::huobi_inverse_swap::HuobiInverseSwapRestClient;
pub use exchanges::huobi::huobi_linear_swap::HuobiLinearSwapRestClient;
pub use exchanges::huobi::huobi_option::HuobiOptionRestClient;
pub use exchanges::huobi::huobi_spot::HuobiSpotRestClient;
pub use exchanges::kraken::KrakenRestClient;
pub use exchanges::mxc::mxc_spot::MXCSpotRestClient;
pub use exchanges::mxc::mxc_swap::MXCSwapRestClient;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
