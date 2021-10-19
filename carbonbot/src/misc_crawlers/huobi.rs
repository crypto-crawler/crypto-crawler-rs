use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::{MarketType, Message, MessageType};
use crypto_ws_client::*;

pub(super) fn crawl_other(market_type: MarketType, tx: Sender<Message>, duration: Option<u64>) {
    let tx = create_conversion_thread("huobi".to_string(), MessageType::Other, market_type, tx);
    let channels: Vec<String> = vec!["market.overview".to_string()];

    match market_type {
        MarketType::Spot => {
            let ws_client = HuobiSpotWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::InverseFuture => {
            let ws_client = HuobiFutureWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::LinearSwap => {
            let ws_client = HuobiLinearSwapWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::InverseSwap => {
            let ws_client = HuobiInverseSwapWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::EuropeanOption => {
            let ws_client = HuobiOptionWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Unknown market_type {}", market_type),
    };
}
