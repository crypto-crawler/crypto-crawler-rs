mod crawlers;
mod market_type;
mod msg;

pub use market_type::MarketType;
pub use msg::*;

pub(crate) const SNAPSHOT_INTERVAL: u64 = 30; // 30 seconds

/// Crawl trades.
pub fn crawl_trade<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "Binance" => crawlers::binance::crawl_trade,
        "Bitfinex" => crawlers::bitfinex::crawl_trade,
        "BitMEX" => crawlers::bitmex::crawl_trade,
        "Bitstamp" => crawlers::bitstamp::crawl_trade,
        "CoinbasePro" => crawlers::coinbase_pro::crawl_trade,
        "Huobi" => crawlers::huobi::crawl_trade,
        "Kraken" => crawlers::kraken::crawl_trade,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, duration);
}

/// Crawl level2 orderbook update events.
pub fn crawl_l2_event<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "Binance" => crawlers::binance::crawl_l2_event,
        "Bitfinex" => crawlers::bitfinex::crawl_l2_event,
        "BitMEX" => crawlers::bitmex::crawl_l2_event,
        "Bitstamp" => crawlers::bitstamp::crawl_l2_event,
        "CoinbasePro" => crawlers::coinbase_pro::crawl_l2_event,
        "Huobi" => crawlers::huobi::crawl_l2_event,
        "Kraken" => crawlers::kraken::crawl_l2_event,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, duration);
}

/// Crawl level2 orderbook snapshot through REST API.
pub fn crawl_l2_snapshot<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "Binance" => crawlers::binance::crawl_l2_snapshot,
        "Bitfinex" => crawlers::bitfinex::crawl_l2_snapshot,
        "BitMEX" => crawlers::bitmex::crawl_l2_snapshot,
        "Bitstamp" => crawlers::bitstamp::crawl_l2_snapshot,
        "CoinbasePro" => crawlers::coinbase_pro::crawl_l2_snapshot,
        "Huobi" => crawlers::huobi::crawl_l2_snapshot,
        "Kraken" => crawlers::kraken::crawl_l2_snapshot,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, duration);
}

/// Crawl level3 orderbook update events.
pub fn crawl_l3_event<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "Binance" => panic!("Binance does NOT provide level3 orderbook data"),
        "Bitfinex" => crawlers::bitfinex::crawl_l3_event,
        "Bitstamp" => crawlers::bitstamp::crawl_l3_event,
        "CoinbasePro" => crawlers::coinbase_pro::crawl_l3_event,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, duration);
}

/// Crawl level3 orderbook snapshot through REST API.
pub fn crawl_l3_snapshot<'a>(
    exchange: &str,
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "Binance" => panic!("Binance does NOT provide level3 orderbook data"),
        "Bitfinex" => crawlers::bitfinex::crawl_l3_snapshot,
        "Bitstamp" => crawlers::bitstamp::crawl_l3_snapshot,
        "CoinbasePro" => crawlers::coinbase_pro::crawl_l3_snapshot,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, duration);
}
