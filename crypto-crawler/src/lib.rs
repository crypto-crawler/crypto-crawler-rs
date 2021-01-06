mod crawlers;
mod market_type;
mod msg;

pub use market_type::MarketType;
pub use msg::*;

/// Crawl trades.
///
/// `market_type` specifies the market type, true means contract markets
/// including future, swap and option, false means spot market.
pub fn crawl_trade<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    match exchange {
        "Binance" => crawlers::binance::crawl_trade(market_type, symbols, on_msg, duration),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
