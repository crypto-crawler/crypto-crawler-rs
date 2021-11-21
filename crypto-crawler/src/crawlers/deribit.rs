use super::crawl_event;
use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "deribit";

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if symbols.is_none() || symbols.unwrap().is_empty() {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Trade,
            market_type,
            tx,
        );

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

        let ws_client = DeribitWSClient::new(tx, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
    } else {
        crawl_event(
            EXCHANGE_NAME,
            MessageType::Trade,
            market_type,
            symbols,
            tx,
            duration,
        );
    }
}
