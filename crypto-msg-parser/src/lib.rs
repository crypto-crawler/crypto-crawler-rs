mod exchanges;
mod msg;

pub use msg::*;

pub use crypto_market_type::MarketType;

use serde_json::Result;

/// Parse trade messages.
pub fn parse_trade(exchange: &str, market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match exchange {
        "binance" => exchanges::binance::parse_trade(market_type, msg),
        "bitfinex" => exchanges::bitfinex::parse_trade(market_type, msg),
        "bitget" => exchanges::bitget::parse_trade(market_type, msg),
        "bithumb" => exchanges::bithumb::parse_trade(market_type, msg),
        "bitmex" => exchanges::bitmex::parse_trade(market_type, msg),
        "bitstamp" => exchanges::bitstamp::parse_trade(market_type, msg),
        "bitz" => exchanges::bitz::parse_trade(market_type, msg),
        "bybit" => exchanges::bybit::parse_trade(market_type, msg),
        "coinbase_pro" => exchanges::coinbase_pro::parse_trade(market_type, msg),
        "deribit" => exchanges::deribit::parse_trade(market_type, msg),
        "ftx" => exchanges::ftx::parse_trade(market_type, msg),
        "gate" => exchanges::gate::parse_trade(market_type, msg),
        "huobi" => exchanges::huobi::parse_trade(market_type, msg),
        "kraken" => exchanges::kraken::parse_trade(market_type, msg),
        "kucoin" => exchanges::kucoin::parse_trade(market_type, msg),
        "mxc" => exchanges::mxc::parse_trade(market_type, msg),
        "okex" => exchanges::okex::parse_trade(market_type, msg),
        "zbg" => exchanges::zbg::parse_trade(market_type, msg),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

/// Parse funding rate messages.
pub fn parse_funding_rate(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let func = match exchange {
        "binance" => exchanges::binance::parse_funding_rate,
        "bitget" => exchanges::bitget::parse_funding_rate,
        "bitmex" => exchanges::bitmex::parse_funding_rate,
        "huobi" => exchanges::huobi::parse_funding_rate,
        "okex" => exchanges::okex::parse_funding_rate,
        _ => panic!("{} does NOT have perpetual swap market", exchange),
    };
    func(market_type, msg)
}
