use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

pub(super) async fn crawl_other(market_type: MarketType, tx: Sender<Message>) {
    let tx = create_conversion_thread("bybit".to_string(), MessageType::Other, market_type, tx);
    let commands = vec![r#"{"op":"subscribe","args":["insurance","liquidation"]}"#.to_string()];

    match market_type {
        MarketType::InverseFuture | MarketType::InverseSwap => {
            let ws_client = BybitInverseWSClient::new(tx, None).await;
            ws_client.send(&commands).await;
            ws_client.run().await;
            ws_client.close();
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}
