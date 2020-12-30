mod exchanges;

pub use exchanges::binance::BinanceClient;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
