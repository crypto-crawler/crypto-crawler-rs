use core::panic;
use std::sync::mpsc::Sender;

use crate::crawlers::utils::{crawl_candlestick_ext, crawl_event};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;

use super::utils::create_conversion_thread;

const EXCHANGE_NAME: &str = "binance";

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbols = symbols
        .unwrap_or_default()
        .iter()
        .map(|symbol| symbol.to_lowercase())
        .collect::<Vec<String>>();
    let symbols = if symbols.is_empty() {
        None
    } else {
        Some(symbols.as_slice())
    };
    if market_type == MarketType::EuropeanOption
        && (symbols.is_none() || symbols.unwrap().is_empty())
    {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Trade,
            market_type,
            tx,
        );
        let channels: Vec<String> = vec![
            "BTCUSDT_C@TRADE_ALL".to_string(),
            "BTCUSDT_P@TRADE_ALL".to_string(),
        ];

        let ws_client = BinanceOptionWSClient::new(tx, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::Trade,
            market_type,
            symbols,
            tx,
            duration,
        )
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbols = symbols
        .unwrap_or_default()
        .iter()
        .map(|symbol| symbol.to_lowercase())
        .collect::<Vec<String>>();
    let symbols = if symbols.is_empty() {
        None
    } else {
        Some(symbols.as_slice())
    };
    crawl_event(
        EXCHANGE_NAME,
        MessageType::L2Event,
        market_type,
        symbols,
        tx,
        duration,
    );
}

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbols = symbols
        .unwrap_or_default()
        .iter()
        .map(|symbol| symbol.to_lowercase())
        .collect::<Vec<String>>();
    let symbols = if symbols.is_empty() {
        None
    } else {
        Some(symbols.as_slice())
    };
    if symbols.is_none() || symbols.unwrap().is_empty() {
        let tx =
            create_conversion_thread(EXCHANGE_NAME.to_string(), MessageType::BBO, market_type, tx);
        let channels = vec!["!bookTicker".to_string()]; // All Book Tickers Stream
        match market_type {
            MarketType::Spot => {
                let ws_client = BinanceSpotWSClient::new(tx, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            MarketType::InverseFuture | MarketType::InverseSwap => {
                let ws_client = BinanceInverseWSClient::new(tx, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            MarketType::LinearFuture | MarketType::LinearSwap => {
                let ws_client = BinanceLinearWSClient::new(tx, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            _ => panic!(
                "Binance {} market does NOT have the BBO channel",
                market_type
            ),
        }
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::BBO,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}

pub(crate) fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbols = symbols
        .unwrap_or_default()
        .iter()
        .map(|symbol| symbol.to_lowercase())
        .collect::<Vec<String>>();
    let symbols = if symbols.is_empty() {
        None
    } else {
        Some(symbols.as_slice())
    };
    crawl_event(
        EXCHANGE_NAME,
        MessageType::L2TopK,
        market_type,
        symbols,
        tx,
        duration,
    );
}

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbols = symbols
        .unwrap_or_default()
        .iter()
        .map(|symbol| symbol.to_lowercase())
        .collect::<Vec<String>>();
    let symbols = if symbols.is_empty() {
        None
    } else {
        Some(symbols.as_slice())
    };

    if symbols.is_none() || symbols.unwrap().is_empty() {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Ticker,
            market_type,
            tx,
        );
        let channels: Vec<String> = vec!["!ticker@arr".to_string()];

        match market_type {
            MarketType::Spot => {
                let ws_client = BinanceSpotWSClient::new(tx, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            MarketType::InverseFuture | MarketType::InverseSwap => {
                let ws_client = BinanceInverseWSClient::new(tx, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            MarketType::LinearFuture | MarketType::LinearSwap => {
                let ws_client = BinanceLinearWSClient::new(tx, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
            }
            _ => panic!(
                "Binance {} market does NOT have the ticker channel",
                market_type
            ),
        }
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::Ticker,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbols = symbols
        .unwrap_or_default()
        .iter()
        .map(|symbol| symbol.to_lowercase())
        .collect::<Vec<String>>();
    let symbols = if symbols.is_empty() {
        None
    } else {
        Some(symbols.as_slice())
    };

    let channels: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        vec!["!markPrice@arr".to_string()]
    } else {
        symbols
            .unwrap()
            .iter()
            .map(|symbol| format!("{}@markPrice", symbol))
            .collect()
    };

    let tx = create_conversion_thread(
        EXCHANGE_NAME.to_string(),
        MessageType::FundingRate,
        market_type,
        tx,
    );

    match market_type {
        MarketType::InverseSwap => {
            let ws_client = BinanceInverseWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::LinearSwap => {
            let ws_client = BinanceLinearWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Binance {} does NOT have funding rates", market_type),
    }
}

pub(crate) fn crawl_candlestick(
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    // All symbols for websocket are lowercase while for REST they are uppercase
    let symbol_interval_list = symbol_interval_list
        .unwrap_or_default()
        .iter()
        .map(|(symbol, interval)| (symbol.to_lowercase(), *interval))
        .collect::<Vec<(String, usize)>>();
    let symbol_interval_list = if symbol_interval_list.is_empty() {
        None
    } else {
        Some(symbol_interval_list.as_slice())
    };
    crawl_candlestick_ext(
        EXCHANGE_NAME,
        market_type,
        symbol_interval_list,
        tx,
        duration,
    );
}
