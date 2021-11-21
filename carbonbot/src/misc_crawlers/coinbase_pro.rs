use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

pub(super) fn crawl_other(market_type: MarketType, tx: Sender<Message>, duration: Option<u64>) {
    let tx = create_conversion_thread(
        "coinbase_pro".to_string(),
        MessageType::Other,
        market_type,
        tx,
    );
    let channels: Vec<String> =
        vec![r#"{"type": "subscribe","channels":[{ "name": "status"}]}"#.to_string()];

    let ws_client = CoinbaseProWSClient::new(tx, None);
    ws_client.subscribe(&channels);
    ws_client.run(duration);
}
