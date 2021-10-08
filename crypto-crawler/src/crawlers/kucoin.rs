use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use std::sync::{Arc, Mutex};

use super::crawl_event;

const EXCHANGE_NAME: &str = "kucoin";

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    if market_type == MarketType::Spot && (symbols.is_none() || symbols.unwrap().is_empty()) {
        let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                market_type,
                MessageType::BBO,
                msg,
            );
            (on_msg.lock().unwrap())(message);
        }));

        // https://docs.kucoin.com/#all-symbols-ticker
        let channels: Vec<String> = vec!["/market/ticker:all".to_string()];

        let ws_client = KuCoinSpotWSClient::new(on_msg_ext, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::BBO,
            market_type,
            symbols,
            on_msg,
            duration,
        );
    }
}
