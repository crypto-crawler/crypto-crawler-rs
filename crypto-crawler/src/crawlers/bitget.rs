use super::utils::fetch_symbols_retry;
use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "bitget";

#[allow(clippy::unnecessary_unwrap)]
pub(crate) async fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    let tx = create_conversion_thread(
        EXCHANGE_NAME.to_string(),
        MessageType::FundingRate,
        market_type,
        tx,
    );

    let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        tokio::task::block_in_place(move || fetch_symbols_retry(EXCHANGE_NAME, market_type))
    } else {
        symbols
            .unwrap()
            .iter()
            .map(|symbol| symbol.to_string())
            .collect()
    };
    let topics: Vec<(String, String)> = symbols
        .into_iter()
        .map(|symbol| ("funding_rate".to_string(), symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_client = BitgetSwapWSClient::new(tx, None).await;
            ws_client.subscribe(&topics).await;
            ws_client.run().await;
            ws_client.close();
        }
        _ => panic!("Bitget {} does NOT have funding rates", market_type),
    }
}
