use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::{MarketType, Message, MessageType};
use crypto_ws_client::*;

pub(super) fn crawl_other(market_type: MarketType, tx: Sender<Message>, duration: Option<u64>) {
    let tx = create_conversion_thread("bybit".to_string(), MessageType::Other, market_type, tx);
    let channels: Vec<String> = vec!["insurance".to_string(), "liquidation".to_string()];

    match market_type {
        MarketType::InverseFuture => {
            let ws_client = BybitInverseFutureWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::InverseSwap => {
            let ws_client = BybitInverseSwapWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}
