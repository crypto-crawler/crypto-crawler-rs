mod exchanges;
mod msg;
mod order;

pub use msg::*;
pub use order::Order;

pub use crypto_market_type::MarketType;

use serde_json::Result;

/// Extract the symbol from the message.
pub fn extract_symbol(exchange: &str, market_type: MarketType, msg: &str) -> Option<String> {
    match exchange {
        "binance" => exchanges::binance::extract_symbol(market_type, msg),
        "bitfinex" => exchanges::bitfinex::extract_symbol(market_type, msg),
        "bitget" => exchanges::bitget::extract_symbol(market_type, msg),
        "bithumb" => exchanges::bithumb::extract_symbol(market_type, msg),
        "bitmex" => exchanges::bitmex::extract_symbol(market_type, msg),
        "bitstamp" => exchanges::bitstamp::extract_symbol(market_type, msg),
        "bitz" => exchanges::bitz::extract_symbol(market_type, msg),
        "bybit" => exchanges::bybit::extract_symbol(market_type, msg),
        "coinbase_pro" => exchanges::coinbase_pro::extract_symbol(market_type, msg),
        "deribit" => exchanges::deribit::extract_symbol(market_type, msg),
        "ftx" => exchanges::ftx::extract_symbol(market_type, msg),
        "gate" => exchanges::gate::extract_symbol(market_type, msg),
        "huobi" => exchanges::huobi::extract_symbol(market_type, msg),
        "kraken" => exchanges::kraken::extract_symbol(market_type, msg),
        "kucoin" => exchanges::kucoin::extract_symbol(market_type, msg),
        "mxc" => exchanges::mxc::extract_symbol(market_type, msg),
        "okex" => exchanges::okex::extract_symbol(market_type, msg),
        "zbg" => exchanges::zbg::extract_symbol(market_type, msg),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

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

/// Parse level2 orderbook messages.
pub fn parse_l2(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>> {
    let ret = match exchange {
        "binance" => exchanges::binance::parse_l2(market_type, msg),
        "bitfinex" => exchanges::bitfinex::parse_l2(
            market_type,
            msg,
            timestamp.expect("Bitfinex orderbook messages doesn't have timestamp"),
        ),
        "bitget" => exchanges::bitget::parse_l2(market_type, msg),
        "bithumb" => exchanges::bithumb::parse_l2(market_type, msg),
        "bitmex" => exchanges::bitmex::parse_l2(
            market_type,
            msg,
            timestamp.expect("BitMEX orderbook messages don't have timestamp"),
        ),
        "bitstamp" => exchanges::bitstamp::parse_l2(market_type, msg),
        "bitz" => exchanges::bitz::parse_l2(market_type, msg),
        "bybit" => exchanges::bybit::parse_l2(market_type, msg),
        "coinbase_pro" => exchanges::coinbase_pro::parse_l2(market_type, msg, timestamp),
        "deribit" => exchanges::deribit::parse_l2(market_type, msg),
        "ftx" => exchanges::ftx::parse_l2(market_type, msg),
        "gate" => exchanges::gate::parse_l2(market_type, msg, timestamp),
        "huobi" => exchanges::huobi::parse_l2(market_type, msg),
        "kraken" => exchanges::kraken::parse_l2(market_type, msg),
        "kucoin" => exchanges::kucoin::parse_l2(market_type, msg, timestamp),
        "mxc" => exchanges::mxc::parse_l2(market_type, msg, timestamp),
        "okex" => exchanges::okex::parse_l2(market_type, msg),
        "zbg" => exchanges::zbg::parse_l2(market_type, msg),
        _ => panic!("Unknown exchange {}", exchange),
    };
    match ret {
        Ok(mut orderbooks) => {
            for orderbook in orderbooks.iter_mut() {
                if orderbook.snapshot {
                    // sorted in ascending order by price
                    orderbook
                        .asks
                        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                    // sorted in descending order by price
                    orderbook
                        .bids
                        .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
                }
            }
            Ok(orderbooks)
        }
        Err(_) => ret,
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
