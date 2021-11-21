use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

pub(super) fn crawl_other(market_type: MarketType, tx: Sender<Message>, duration: Option<u64>) {
    let tx = create_conversion_thread("binance".to_string(), MessageType::Other, market_type, tx);
    let channels: Vec<String> = vec!["!forceOrder@arr".to_string()];

    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            let ws_client = BinanceInverseWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::LinearSwap | MarketType::LinearFuture => {
            let ws_client = BinanceLinearWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}
