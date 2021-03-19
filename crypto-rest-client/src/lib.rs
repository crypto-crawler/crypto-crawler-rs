mod error;
mod exchanges;

pub use error::Error;
pub use exchanges::binance::binance_inverse::BinanceInverseRestClient;
pub use exchanges::binance::binance_linear::BinanceLinearRestClient;
pub use exchanges::binance::binance_option::BinanceOptionRestClient;
pub use exchanges::binance::binance_spot::BinanceSpotRestClient;
pub use exchanges::bitfinex::BitfinexRestClient;
pub use exchanges::bitget::*;
pub use exchanges::bithumb::*;
pub use exchanges::bitmex::BitmexRestClient;
pub use exchanges::bitstamp::BitstampRestClient;
pub use exchanges::bitz::*;
pub use exchanges::bybit::BybitRestClient;
pub use exchanges::coinbase_pro::CoinbaseProRestClient;
pub use exchanges::deribit::DeribitRestClient;
pub use exchanges::ftx::FtxRestClient;
pub use exchanges::gate::*;
pub use exchanges::huobi::huobi_future::HuobiFutureRestClient;
pub use exchanges::huobi::huobi_inverse_swap::HuobiInverseSwapRestClient;
pub use exchanges::huobi::huobi_linear_swap::HuobiLinearSwapRestClient;
pub use exchanges::huobi::huobi_option::HuobiOptionRestClient;
pub use exchanges::huobi::huobi_spot::HuobiSpotRestClient;
pub use exchanges::kraken::KrakenRestClient;
pub use exchanges::kucoin::*;
pub use exchanges::mxc::mxc_spot::MxcSpotRestClient;
pub use exchanges::mxc::mxc_swap::MxcSwapRestClient;
pub use exchanges::okex::OkexRestClient;
pub use exchanges::zbg::*;

use crypto_market_type::MarketType;
use error::Result;

/// Fetch level2 orderbook snapshot.
pub fn fetch_l2_snapshot(exchange: &str, market_type: MarketType, symbol: &str) -> Result<String> {
    match exchange {
        "binance" => exchanges::binance::fetch_l2_snapshot(market_type, symbol),
        "bitfinex" => exchanges::bitfinex::BitfinexRestClient::fetch_l2_snapshot(symbol),
        "bitget" => exchanges::bitget::fetch_l2_snapshot(market_type, symbol),
        "bithumb" => exchanges::bithumb::BithumbRestClient::fetch_l2_snapshot(symbol),
        "bitmex" => exchanges::bitmex::BitmexRestClient::fetch_l2_snapshot(symbol),
        "bitstamp" => exchanges::bitstamp::BitstampRestClient::fetch_l2_snapshot(symbol),
        "bitz" => exchanges::bitz::fetch_l2_snapshot(market_type, symbol),
        "bybit" => exchanges::bybit::BybitRestClient::fetch_l2_snapshot(symbol),
        "coinbase_pro" => exchanges::coinbase_pro::CoinbaseProRestClient::fetch_l2_snapshot(symbol),
        "deribit" => exchanges::deribit::DeribitRestClient::fetch_l2_snapshot(symbol),
        "ftx" => exchanges::ftx::FtxRestClient::fetch_l2_snapshot(symbol),
        "gate" => exchanges::gate::fetch_l2_snapshot(market_type, symbol),
        "huobi" => exchanges::huobi::fetch_l2_snapshot(market_type, symbol),
        "kraken" => exchanges::kraken::KrakenRestClient::fetch_l2_snapshot(symbol),
        "kucoin" => exchanges::kucoin::fetch_l2_snapshot(market_type, symbol),
        "mxc" => exchanges::mxc::fetch_l2_snapshot(market_type, symbol),
        "okex" => exchanges::okex::OkexRestClient::fetch_l2_snapshot(symbol),
        "zbg" => exchanges::zbg::fetch_l2_snapshot(market_type, symbol),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

/// Fetch level3 orderbook snapshot.
pub fn fetch_l3_snapshot(exchange: &str, market_type: MarketType, symbol: &str) -> Result<String> {
    match exchange {
        "bitfinex" => exchanges::bitfinex::BitfinexRestClient::fetch_l3_snapshot(symbol),
        "bitstamp" => exchanges::bitstamp::BitstampRestClient::fetch_l3_snapshot(symbol),
        "coinbase_pro" => exchanges::coinbase_pro::CoinbaseProRestClient::fetch_l3_snapshot(symbol),
        "kucoin" => exchanges::kucoin::fetch_l3_snapshot(market_type, symbol),
        _ => panic!(
            "{} {} does NOT provide level3 orderbook data",
            exchange, market_type
        ),
    }
}
