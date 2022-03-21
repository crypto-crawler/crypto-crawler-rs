use super::crawl_event;
use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "deribit";

pub(crate) async fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if symbols.is_none() || symbols.unwrap().is_empty() {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Trade,
            market_type,
            tx,
        );

        // "any" menas all, see https://docs.deribit.com/?javascript#trades-kind-currency-interval
        let topics: Vec<(String, String)> = match market_type {
            MarketType::InverseFuture => {
                vec![("trades.future.SYMBOL.100ms".to_string(), "any".to_string())]
            }
            MarketType::InverseSwap => {
                vec![
                    (
                        "trades.SYMBOL.100ms".to_string(),
                        "BTC-PERPETUAL".to_string(),
                    ),
                    (
                        "trades.SYMBOL.100ms".to_string(),
                        "ETH-PERPETUAL".to_string(),
                    ),
                ]
            }
            MarketType::EuropeanOption => {
                vec![("trades.option.SYMBOL.100ms".to_string(), "any".to_string())]
            }
            _ => panic!("Deribit does NOT have the {} market type", market_type),
        };

        let ws_client = DeribitWSClient::new(tx, None).await;
        ws_client.subscribe(&topics).await;
        ws_client.run().await;
        ws_client.close();
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::Trade, market_type, symbols, tx).await;
    }
}
