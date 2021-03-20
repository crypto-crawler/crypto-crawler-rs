mod exchanges;
mod msg;

pub use msg::*;

pub use crypto_market_type::MarketType;

use serde_json::Result;

pub fn parse_trade(exchange: &str, market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match exchange {
        "binance" => exchanges::binance::parse_trade(market_type, msg),
        "bitfinex" => exchanges::bitfinex::parse_trade(market_type, msg),
        "bitget" => exchanges::bitget::parse_trade(market_type, msg),
        "huobi" => exchanges::huobi::parse_trade(market_type, msg),
        _ => panic!("Unknown exchange {}", exchange),
    }
}
