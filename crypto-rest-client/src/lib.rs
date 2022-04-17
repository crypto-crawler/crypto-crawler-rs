mod error;
mod exchanges;

pub use error::Error;
pub use exchanges::{
    binance::{
        binance_inverse::BinanceInverseRestClient, binance_linear::BinanceLinearRestClient,
        binance_option::BinanceOptionRestClient, binance_spot::BinanceSpotRestClient,
    },
    bitfinex::BitfinexRestClient,
    bitget::*,
    bithumb::*,
    bitmex::BitmexRestClient,
    bitstamp::BitstampRestClient,
    bitz::*,
    bybit::BybitRestClient,
    coinbase_pro::CoinbaseProRestClient,
    deribit::DeribitRestClient,
    dydx::dydx_swap::DydxSwapRestClient,
    ftx::FtxRestClient,
    gate::*,
    huobi::{
        huobi_future::HuobiFutureRestClient, huobi_inverse_swap::HuobiInverseSwapRestClient,
        huobi_linear_swap::HuobiLinearSwapRestClient, huobi_option::HuobiOptionRestClient,
        huobi_spot::HuobiSpotRestClient,
    },
    kraken::{kraken_futures::KrakenFuturesRestClient, kraken_spot::KrakenSpotRestClient},
    kucoin::*,
    mexc::{mexc_spot::MexcSpotRestClient, mexc_swap::MexcSwapRestClient},
    okx::OkxRestClient,
    zbg::*,
};

use crypto_market_type::MarketType;
use error::Result;
use log::*;
use std::time::{Duration, SystemTime};

fn fetch_l2_snapshot_internal(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
) -> Result<String> {
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
        "dydx" => exchanges::dydx::fetch_l2_snapshot(market_type, symbol),
        "ftx" => exchanges::ftx::FtxRestClient::fetch_l2_snapshot(symbol),
        "gate" => exchanges::gate::fetch_l2_snapshot(market_type, symbol),
        "huobi" => exchanges::huobi::fetch_l2_snapshot(market_type, symbol),
        "kraken" => exchanges::kraken::fetch_l2_snapshot(market_type, symbol),
        "kucoin" => exchanges::kucoin::fetch_l2_snapshot(market_type, symbol),
        "mexc" => exchanges::mexc::fetch_l2_snapshot(market_type, symbol),
        "okx" => exchanges::okx::OkxRestClient::fetch_l2_snapshot(symbol),
        "zbg" => exchanges::zbg::fetch_l2_snapshot(market_type, symbol),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

pub fn fetch_l3_snapshot_internal(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
) -> Result<String> {
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

/// Fetch open interest.
///
/// `symbol` None means fetch all symbols.
pub fn fetch_open_interest(
    exchange: &str,
    market_type: MarketType,
    symbol: Option<&str>,
) -> Result<String> {
    match exchange {
        "binance" => exchanges::binance::fetch_open_interest(market_type, symbol.unwrap()),
        "bitget" => exchanges::bitget::fetch_open_interest(market_type, symbol.unwrap()),
        "bybit" => exchanges::bybit::BybitRestClient::fetch_open_interest(symbol.unwrap()),
        "bitz" => exchanges::bitz::fetch_open_interest(market_type, symbol),
        "deribit" => exchanges::deribit::DeribitRestClient::fetch_open_interest(symbol),
        "dydx" => exchanges::dydx::fetch_open_interest(market_type),
        "ftx" => exchanges::ftx::FtxRestClient::fetch_open_interest(),
        "gate" => exchanges::gate::fetch_open_interest(market_type, symbol.unwrap()),
        "huobi" => exchanges::huobi::fetch_open_interest(market_type, symbol),
        "kucoin" => exchanges::kucoin::fetch_open_interest(market_type),
        "okx" => exchanges::okx::OkxRestClient::fetch_open_interest(market_type, symbol),
        "zbg" => exchanges::zbg::fetch_open_interest(market_type, symbol.unwrap()),
        _ => panic!("{} does NOT have open interest RESTful API", exchange),
    }
}

/// Fetch level2 orderbook snapshot.
///
/// `retry` None means no retry; Some(0) means retry unlimited times; Some(n) means retry n times.
pub fn fetch_l2_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
    retry: Option<u64>,
) -> Result<String> {
    retriable(
        exchange,
        market_type,
        symbol,
        fetch_l2_snapshot_internal,
        retry,
    )
}

/// Fetch level3 orderbook snapshot.
///
/// `retry` None means no retry; Some(0) means retry unlimited times; Some(n) means retry n times.
pub fn fetch_l3_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
    retry: Option<u64>,
) -> Result<String> {
    retriable(
        exchange,
        market_type,
        symbol,
        fetch_l3_snapshot_internal,
        retry,
    )
}

// `retry` None means no retry; Some(0) means retry unlimited times; Some(n) means retry n times.
fn retriable(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
    crawl_func: fn(&str, MarketType, &str) -> Result<String>,
    retry: Option<u64>,
) -> Result<String> {
    let retry_count = {
        let count = retry.unwrap_or(1);
        if count == 0 {
            u64::MAX
        } else {
            count
        }
    };
    if retry_count == 1 {
        return crawl_func(exchange, market_type, symbol);
    }
    let mut backoff_factor = 0;
    let cooldown_time = Duration::from_secs(2);
    for _ in 0..retry_count {
        let resp = crawl_func(exchange, market_type, symbol);
        match resp {
            Ok(msg) => return Ok(msg),
            Err(err) => {
                let current_timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                warn!(
                    "{} {} {} {} {}, error: {}, back off for {} milliseconds",
                    current_timestamp,
                    backoff_factor,
                    exchange,
                    market_type,
                    symbol,
                    err,
                    (backoff_factor * cooldown_time).as_millis()
                );
                std::thread::sleep(backoff_factor * cooldown_time);
                if err.0.contains("429") {
                    backoff_factor += 1;
                } else {
                    // Handle 403, 418, etc.
                    backoff_factor *= 2;
                }
            }
        }
    }
    Err(Error(format!(
        "Failed {} {} {} after retrying {} times",
        exchange, market_type, symbol, retry_count
    )))
}
