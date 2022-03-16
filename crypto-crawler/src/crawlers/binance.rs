use core::panic;
use std::sync::mpsc::Sender;

use crate::crawlers::utils::crawl_event;
use crate::msg::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

use super::utils::create_conversion_thread;

const EXCHANGE_NAME: &str = "binance";

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
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

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
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

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
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
            MarketType::EuropeanOption => {
                let channels: Vec<String> = vec!["BTCUSDT@TICKER_ALL".to_string()];
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
    let channels: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        vec!["!markPrice@arr".to_string()]
    } else {
        symbols
            .unwrap()
            .iter()
            .map(|symbol| format!("{}@markPrice", symbol.to_lowercase()))
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
