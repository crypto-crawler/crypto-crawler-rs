use core::panic;
use std::sync::mpsc::Sender;

use crate::{crawlers::utils::crawl_event, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

use super::utils::create_conversion_thread;

const EXCHANGE_NAME: &str = "binance";

pub(crate) async fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
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
        let topics: Vec<(String, String)> = vec![
            // ("TICKER_ALL".to_string(), "BTCUSDT".to_string()),
            ("TRADE_ALL".to_string(), "BTCUSDT_C".to_string()),
            ("TRADE_ALL".to_string(), "BTCUSDT_P".to_string()),
        ];

        let ws_client = BinanceOptionWSClient::new(tx, None).await;
        ws_client.subscribe(&topics).await;
        ws_client.run().await;
        ws_client.close();
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::Trade, market_type, symbols, tx).await;
    }
}

pub(crate) async fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if symbols.is_none() || symbols.unwrap().is_empty() {
        let tx =
            create_conversion_thread(EXCHANGE_NAME.to_string(), MessageType::BBO, market_type, tx);
        let commands =
            vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]; // All Book Tickers Stream
        match market_type {
            MarketType::Spot => {
                let ws_client = BinanceSpotWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            MarketType::InverseFuture | MarketType::InverseSwap => {
                let ws_client = BinanceInverseWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            MarketType::LinearFuture | MarketType::LinearSwap => {
                let ws_client = BinanceLinearWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            _ => panic!(
                "Binance {} market does NOT have the BBO channel",
                market_type
            ),
        }
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::BBO, market_type, symbols, tx).await;
    }
}

pub(crate) async fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if symbols.is_none() || symbols.unwrap().is_empty() {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Ticker,
            market_type,
            tx,
        );
        let commands = vec![
            r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@depth20@100ms"]}"#.to_string(),
        ];

        match market_type {
            MarketType::Spot => {
                let ws_client = BinanceSpotWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            MarketType::InverseFuture | MarketType::InverseSwap => {
                let ws_client = BinanceInverseWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            MarketType::LinearFuture | MarketType::LinearSwap => {
                let ws_client = BinanceLinearWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            MarketType::EuropeanOption => {
                let commands = vec![
                    r#"{"id":9527,"method":"SUBSCRIBE","params":["BTCUSDT@depth5@100ms"]}"#
                        .to_string(),
                ];
                let ws_client = BinanceLinearWSClient::new(tx, None).await;
                ws_client.send(&commands).await;
                ws_client.run().await;
                ws_client.close();
            }
            _ => panic!(
                "Binance {} market does NOT have the ticker channel",
                market_type
            ),
        }
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::Ticker, market_type, symbols, tx).await;
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) async fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    let tx = create_conversion_thread(
        EXCHANGE_NAME.to_string(),
        MessageType::FundingRate,
        market_type,
        tx,
    );
    let ws_client: Box<dyn WSClient + Send + Sync> = match market_type {
        MarketType::InverseSwap => Box::new(BinanceInverseWSClient::new(tx, None).await),
        MarketType::LinearSwap => Box::new(BinanceLinearWSClient::new(tx, None).await),
        _ => panic!("Binance {} does NOT have funding rates", market_type),
    };

    if symbols.is_none() || symbols.unwrap().is_empty() {
        let commands =
            vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!markPrice@arr"]}"#.to_string()];
        ws_client.send(&commands).await;
    } else {
        let topics = symbols
            .unwrap()
            .iter()
            .map(|symbol| ("markPrice".to_string(), symbol.to_string()))
            .collect::<Vec<(String, String)>>();
        ws_client.subscribe(&topics).await;
    };

    ws_client.run().await;
    ws_client.close();
}
