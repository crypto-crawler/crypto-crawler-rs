mod exchanges;

pub use exchanges::binance::binance_future::BinanceFutureRestClient;
pub use exchanges::binance::binance_inverse_swap::BinanceInverseSwapRestClient;
pub use exchanges::binance::binance_linear_swap::BinanceLinearSwapRestClient;
pub use exchanges::binance::binance_spot::BinanceSpotRestClient;
pub use exchanges::bitfinex::BitfinexRestClient;
pub use exchanges::bitmex::BitMEXRestClient;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
