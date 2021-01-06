mod crawlers;
mod market_type;
mod msg;

pub use market_type::MarketType;
pub use msg::*;

/// Crawl trades.
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

/// Crawl level2 orderbook update events.
pub fn crawl_l2_event<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    match exchange {
        "Binance" => crawlers::binance::crawl_l2_event(market_type, symbols, on_msg, duration),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

/// Crawl level2 orderbook snapshot through REST API.
pub fn crawl_l2_snapshot<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    match exchange {
        "Binance" => crawlers::binance::crawl_l2_snapshot(market_type, symbols, on_msg, duration),
        _ => panic!("Unknown exchange {}", exchange),
    }
}
