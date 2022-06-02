use std::sync::mpsc::Sender;

use crate::{crawlers::utils::crawl_event, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

use super::utils::create_conversion_thread;

const EXCHANGE_NAME: &str = "zbg";

pub(crate) async fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if symbols.is_none() || symbols.unwrap().is_empty() {
        if market_type == MarketType::Spot {
            let tx = create_conversion_thread(
                EXCHANGE_NAME.to_string(),
                MessageType::Ticker,
                market_type,
                tx,
            );
            let commands: Vec<String> =
                vec![r#"{"action":"ADD", "dataType":"ALL_TRADE_STATISTIC_24H"}"#.to_string()];

            let ws_client = ZbgSpotWSClient::new(tx, None).await;
            ws_client.send(&commands).await;
            ws_client.run().await;
            ws_client.close();
        } else {
            let tx = create_conversion_thread(
                EXCHANGE_NAME.to_string(),
                MessageType::Ticker,
                market_type,
                tx,
            );
            let commands: Vec<String> =
                vec![r#"{"action":"sub", "topic":"future_all_indicator"}"#.to_string()];

            let ws_client = ZbgSwapWSClient::new(tx, None).await;
            ws_client.send(&commands).await;
            ws_client.run().await;
            ws_client.close();
        }
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::Ticker, market_type, symbols, tx).await;
    }
}
