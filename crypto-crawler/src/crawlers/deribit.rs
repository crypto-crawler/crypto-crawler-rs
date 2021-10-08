use super::crawl_event;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use std::sync::{Arc, Mutex};

const EXCHANGE_NAME: &str = "deribit";

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if symbols.is_none() || symbols.unwrap().is_empty() {
        let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                market_type,
                MessageType::Trade,
                msg,
            );
            (on_msg.lock().unwrap())(message);
        }));

        // "any" menas all, see https://docs.deribit.com/?javascript#trades-kind-currency-interval
        let channels: Vec<String> = match market_type {
            MarketType::InverseFuture => vec!["trades.future.any.raw"],
            MarketType::InverseSwap => vec!["trades.BTC-PERPETUAL.raw", "trades.ETH-PERPETUAL.raw"],
            MarketType::EuropeanOption => vec!["trades.option.any.raw"],
            _ => panic!("Deribit does NOT have the {} market type", market_type),
        }
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        let ws_client = DeribitWSClient::new(on_msg_ext, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
        None
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::Trade,
            market_type,
            symbols,
            on_msg,
            duration,
        )
    }
}
