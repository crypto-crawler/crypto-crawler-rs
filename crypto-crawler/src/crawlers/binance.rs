use std::collections::HashMap;
use std::future::Future;

use crate::msg::*;
use crypto_markets::{fetch_markets, Market, MarketType};
use lazy_static::lazy_static;


const EXCHANGE_NAME: &str = "Binance";

lazy_static! {
    static ref PERIOD_NAMES: HashMap<&'static str, &'static str> =
        vec![("1m", "1m"), ("3m", "3m"), ("5m", "5m"), ("15m", "15m"),]
            .into_iter()
            .collect();
    static ref WEBSOCKET_ENDPOINTS: HashMap<&'static str, &'static str> = vec![
        ("Spot", "wss://stream.binance.com:9443"),
        ("Swap", "wss://fstream.binance.com"),
        ("Futures", "wss://dstream.binance.com"),
    ]
    .into_iter()
    .collect();
}

fn get_channel(
    market_type: MarketType,
    channel_type: ChannelType,
    pair: &str,
    markets: &[Market],
) -> Vec<String> {
    let market = markets
        .iter()
        .find(|x| x.pair == pair && matches!(x.market_type, market_type))
        .unwrap();
    assert_eq!(market.exchange, EXCHANGE_NAME);

    let raw_pair: &str = &market.id.to_lowercase();
    match channel_type {
        ChannelType::BBO => vec![format!("{}@bookTicker", raw_pair)],
        ChannelType::FundingRate => vec![format!("{}@markPrice", raw_pair)],
        ChannelType::Kline => PERIOD_NAMES
            .keys()
            .into_iter()
            .map(|x| format!("{}@kline_{}", raw_pair, x))
            .collect::<Vec<String>>(),
        ChannelType::OrderBook => vec![format!("{}@depth", raw_pair)],
        ChannelType::Ticker => vec![format!("{}@ticker", raw_pair)],
        ChannelType::Trade => vec![format!("{}@aggTrade", raw_pair)], // trade or aggTrade
    }
}

fn get_channel_type(channel: &str) -> ChannelType {
    assert!(channel.contains('@'));
    let suffix = channel.split('@').nth(1).unwrap();

    match suffix {
        "bookTicker" => ChannelType::BBO,
        "markPrice" => ChannelType::FundingRate,
        "depth" => ChannelType::OrderBook,
        "ticker" => ChannelType::Ticker,
        "trade" => ChannelType::Trade,
        "aggTrade" => ChannelType::Trade,
        _ => {
            if suffix.starts_with("kline_") {
                ChannelType::Kline
            } else {
                panic!("Unknown channel: {}", channel)
            }
        }
    }
}

pub async fn crawl<Fut>(
    market_type: MarketType,
    channel_types: &[ChannelType],
    pairs: &[&str],
    msg_callback: impl Fn(Msg) -> Fut,
) -> ()
where
    Fut: Future<Output = ()>,
{
    // retry 3 times
    let mut markets = Vec::<Market>::new();
    for _i in 0..3 {
        let resp = fetch_markets(EXCHANGE_NAME, market_type);
        if let Ok(m) = resp {
            markets = m;
            break;
        }
        match resp {
            Ok(res) => {
                markets = res;
                break;
            }
            Err(err) => (),
        }
    }
    return;
}
