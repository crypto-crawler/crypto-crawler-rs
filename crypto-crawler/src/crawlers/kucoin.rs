use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

use super::crawl_event;

const EXCHANGE_NAME: &str = "kucoin";

pub(crate) async fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Spot && (symbols.is_none() || symbols.unwrap().is_empty()) {
        let tx =
            create_conversion_thread(EXCHANGE_NAME.to_string(), MessageType::BBO, market_type, tx);

        // https://docs.kucoin.com/#all-symbols-ticker
        let commands: Vec<String> = vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/ticker:all","privateChannel":false,"response":true}"#.to_string()];
        let ws_client = KuCoinSpotWSClient::new(tx, None).await;
        ws_client.send(&commands).await;
        ws_client.run().await;
        ws_client.close().await;
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::BBO, market_type, symbols, tx).await;
    }
}
