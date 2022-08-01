pub mod exchanges;
use crypto_market_type::MarketType;
use crypto_message::{BboMsg, FundingRateMsg, Order, OrderBookMsg, TradeMsg, CandlestickMsg};
use crypto_msg_type::MessageType;
pub use exchanges::utils::round; // for test only
use simple_error::SimpleError;

/// Extract the symbol from the message.
///
/// If the message contains multiple symbols, `ALL` is returned;
/// If the message has no symbol, `NONE` is returned.
pub fn extract_symbol(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
) -> Result<String, SimpleError> {
    match exchange {
        "binance" => exchanges::binance::extract_symbol(msg),
        "bitfinex" => exchanges::bitfinex::extract_symbol(msg),
        "bitget" => exchanges::bitget::extract_symbol(market_type, msg),
        "bithumb" => exchanges::bithumb::extract_symbol(market_type, msg),
        "bitmex" => exchanges::bitmex::extract_symbol(market_type, msg),
        "bitstamp" => exchanges::bitstamp::extract_symbol(market_type, msg),
        "bitz" => exchanges::bitz::extract_symbol(market_type, msg),
        "bybit" => exchanges::bybit::extract_symbol(market_type, msg),
        "coinbase_pro" => exchanges::coinbase_pro::extract_symbol(market_type, msg),
        "deribit" => exchanges::deribit::extract_symbol(market_type, msg),
        "dydx" => exchanges::dydx::extract_symbol(msg),
        "ftx" => exchanges::ftx::extract_symbol(market_type, msg),
        "gate" => exchanges::gate::extract_symbol(market_type, msg),
        "huobi" => exchanges::huobi::extract_symbol(msg),
        "kraken" => exchanges::kraken::extract_symbol(market_type, msg),
        "kucoin" => exchanges::kucoin::extract_symbol(msg),
        "mxc" | "mexc" => exchanges::mexc::extract_symbol(msg),
        "okex" | "okx" => exchanges::okx::extract_symbol(market_type, msg),
        "zb" => exchanges::zb::extract_symbol(market_type, msg),
        "zbg" => exchanges::zbg::extract_symbol(market_type, msg),
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
    }
}

/// Extract the timestamp from the message.
pub fn extract_timestamp(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    match exchange {
        "binance" => exchanges::binance::extract_timestamp(msg),
        "bitfinex" => exchanges::bitfinex::extract_timestamp(msg),
        "bitget" => exchanges::bitget::extract_timestamp(market_type, msg),
        "bithumb" => exchanges::bithumb::extract_timestamp(market_type, msg),
        "bitmex" => exchanges::bitmex::extract_timestamp(market_type, msg),
        "bitstamp" => exchanges::bitstamp::extract_timestamp(market_type, msg),
        "bitz" => exchanges::bitz::extract_timestamp(market_type, msg),
        "bybit" => exchanges::bybit::extract_timestamp(market_type, msg),
        "coinbase_pro" => exchanges::coinbase_pro::extract_timestamp(market_type, msg),
        "deribit" => exchanges::deribit::extract_timestamp(market_type, msg),
        "dydx" => exchanges::dydx::extract_timestamp(market_type, msg),
        "ftx" => exchanges::ftx::extract_timestamp(market_type, msg),
        "gate" => exchanges::gate::extract_timestamp(market_type, msg),
        "huobi" => exchanges::huobi::extract_timestamp(msg),
        "kraken" => exchanges::kraken::extract_timestamp(market_type, msg),
        "kucoin" => exchanges::kucoin::extract_timestamp(msg),
        "mxc" | "mexc" => exchanges::mexc::extract_timestamp(msg),
        "okex" | "okx" => exchanges::okx::extract_timestamp(market_type, msg),
        "zb" => exchanges::zb::extract_timestamp(market_type, msg),
        "zbg" => exchanges::zbg::extract_timestamp(market_type, msg),
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
    }
}

/// Parse trade messages.
pub fn parse_trade(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
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
        "dydx" => exchanges::dydx::parse_trade(market_type, msg),
        "ftx" => exchanges::ftx::parse_trade(market_type, msg),
        "gate" => exchanges::gate::parse_trade(market_type, msg),
        "huobi" => exchanges::huobi::parse_trade(market_type, msg),
        "kraken" => exchanges::kraken::parse_trade(market_type, msg),
        "kucoin" => exchanges::kucoin::parse_trade(market_type, msg),
        "mxc" | "mexc" => exchanges::mexc::parse_trade(market_type, msg),
        "okex" | "okx" => exchanges::okx::parse_trade(market_type, msg),
        "zb" => exchanges::zb::parse_trade(market_type, msg),
        "zbg" => exchanges::zbg::parse_trade(market_type, msg),
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
    }
}

/// Parse level2 orderbook messages.
pub fn parse_l2(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ret = match exchange {
        "binance" => exchanges::binance::parse_l2(market_type, msg),
        "bitfinex" => exchanges::bitfinex::parse_l2(
            market_type,
            msg,
            received_at.expect("Bitfinex orderbook messages doesn't have timestamp"),
        ),
        "bitget" => exchanges::bitget::parse_l2(market_type, msg),
        "bithumb" => exchanges::bithumb::parse_l2(market_type, msg),
        "bitmex" => exchanges::bitmex::parse_l2(
            market_type,
            msg,
            received_at.expect("BitMEX orderbook messages don't have timestamp"),
        ),
        "bitstamp" => exchanges::bitstamp::parse_l2(market_type, msg),
        "bitz" => exchanges::bitz::parse_l2(market_type, msg),
        "bybit" => exchanges::bybit::parse_l2(market_type, msg),
        "coinbase_pro" => exchanges::coinbase_pro::parse_l2(market_type, msg, received_at),
        "deribit" => exchanges::deribit::parse_l2(market_type, msg),
        "dydx" => exchanges::dydx::parse_l2(
            market_type,
            msg,
            received_at.expect("dYdX orderbook messages don't have timestamp"),
        ),
        "ftx" => exchanges::ftx::parse_l2(market_type, msg),
        "gate" => exchanges::gate::parse_l2(market_type, msg, received_at),
        "huobi" => exchanges::huobi::parse_l2(market_type, msg),
        "kraken" => exchanges::kraken::parse_l2(market_type, msg),
        "kucoin" => exchanges::kucoin::parse_l2(market_type, msg, received_at),
        "mxc" | "mexc" => exchanges::mexc::parse_l2(market_type, msg, received_at),
        "okex" | "okx" => exchanges::okx::parse_l2(market_type, msg),
        "zb" => exchanges::zb::parse_l2(market_type, msg),
        "zbg" => exchanges::zbg::parse_l2(market_type, msg),
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
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

/// Parse level2 topk orderbook messages.
pub fn parse_l2_topk(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ret = match exchange {
        "binance" => exchanges::binance::parse_l2_topk(market_type, msg, received_at),
        "bitget" => exchanges::bitget::parse_l2_topk(market_type, msg),
        "bitmex" => exchanges::bitmex::parse_l2_topk(market_type, msg),
        "bitstamp" => exchanges::bitstamp::parse_l2_topk(market_type, msg),
        "deribit" => exchanges::deribit::parse_l2_topk(market_type, msg),
        "huobi" => exchanges::huobi::parse_l2_topk(market_type, msg),
        "kucoin" => exchanges::kucoin::parse_l2_topk(market_type, msg),
        "mexc" => exchanges::mexc::parse_l2_topk(market_type, msg, received_at),
        "okx" => exchanges::okx::parse_l2_topk(market_type, msg),
        "zb" => exchanges::zb::parse_l2_topk(market_type, msg),
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
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

/// Parse BBO(best bid&offer) messages.
pub fn parse_bbo(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    match exchange {
        "binance" => exchanges::binance::parse_bbo(market_type, msg, received_at),
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
    }
}

/// Parse funding rate messages.
pub fn parse_funding_rate(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    if market_type != MarketType::InverseSwap
        && market_type != MarketType::LinearSwap
        && market_type != MarketType::QuantoSwap
        && market_type != MarketType::Unknown
    {
        return Err(SimpleError::new(
            "Only InverseSwap, LinearSwap and QuantoSwap markets have funding rate.",
        ));
    }
    match exchange {
        "binance" => exchanges::binance::parse_funding_rate(market_type, msg),
        "bitget" => exchanges::bitget::parse_funding_rate(market_type, msg),
        "bitmex" => exchanges::bitmex::parse_funding_rate(
            market_type,
            msg,
            received_at.expect("BitMEX funding rate messages don't have timestamp"),
        ),
        "huobi" => exchanges::huobi::parse_funding_rate(market_type, msg),
        "okex" | "okx" => exchanges::okx::parse_funding_rate(
            market_type,
            msg,
            received_at.expect("OKX funding rate messages don't have timestamp"),
        ),
        _ => Err(SimpleError::new(format!(
            "{} does NOT have perpetual swap market",
            exchange
        ))),
    }
}

pub fn parse_candlestick(
    exchange: &str,
    market_type: MarketType,
    msg: &str,
    msg_type: MessageType
) -> Result<CandlestickMsg, SimpleError> {
    match exchange {
        _ => Err(SimpleError::new(format!("Unknown exchange {}", exchange))),
    }
}

/// Infer the message type from the message.
pub fn get_msg_type(exchange: &str, msg: &str) -> MessageType {
    match exchange {
        "binance" => exchanges::binance::get_msg_type(msg),
        "bitget" => exchanges::bitget::get_msg_type(msg),
        "bitmex" => exchanges::bitmex::get_msg_type(msg),
        "bybit" => exchanges::bybit::get_msg_type(msg),
        "deribit" => exchanges::deribit::get_msg_type(msg),
        "ftx" => exchanges::ftx::get_msg_type(msg),
        "huobi" => exchanges::huobi::get_msg_type(msg),
        "kraken" => exchanges::kraken::get_msg_type(msg),
        "okex" | "okx" => exchanges::okx::get_msg_type(msg),
        _ => MessageType::Other,
    }
}
