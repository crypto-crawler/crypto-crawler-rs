use crate::{crawlers::utils::create_conversion_thread, msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

use super::crawl_event;

const EXCHANGE_NAME: &str = "kucoin";

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Spot && (symbols.is_none() || symbols.unwrap().is_empty()) {
        let tx =
            create_conversion_thread(EXCHANGE_NAME.to_string(), MessageType::BBO, market_type, tx);

        // https://docs.kucoin.com/#all-symbols-ticker
        let channels: Vec<String> = vec!["/market/ticker:all".to_string()];

        let ws_client = KuCoinSpotWSClient::new(tx, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
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
