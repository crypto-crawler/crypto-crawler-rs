use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::{MarketType, Message, MessageType};
use crypto_ws_client::*;

pub(super) fn crawl_other(market_type: MarketType, tx: Sender<Message>, duration: Option<u64>) {
    assert_eq!(market_type, MarketType::Unknown);
    let tx = create_conversion_thread("bitmex".to_string(), MessageType::Other, market_type, tx);
    let channels: Vec<String> = vec![
        "announcement",
        "connected",
        "instrument",
        "insurance",
        "liquidation",
        "publicNotifications",
        "settlement",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    let ws_client = BitmexWSClient::new(tx, None);
    ws_client.subscribe(&channels);
    ws_client.run(duration);
    ws_client.close();
}
