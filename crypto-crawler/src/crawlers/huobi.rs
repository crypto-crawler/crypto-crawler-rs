use super::utils::fetch_symbols_retry;
use crate::crawlers::crawl_event;
use crate::crawlers::utils::create_conversion_thread;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "huobi";

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match market_type {
        MarketType::Spot => {
            let tx = create_conversion_thread(
                EXCHANGE_NAME.to_string(),
                MessageType::L2Event,
                market_type,
                tx,
            );
            let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
                fetch_symbols_retry(EXCHANGE_NAME, market_type)
            } else {
                symbols.unwrap().to_vec()
            };
            // Huobi Spot market.$symbol.mbp.$levels must use wss://api.huobi.pro/feed
            // or wss://api-aws.huobi.pro/feed
            let ws_client = HuobiSpotWSClient::new(tx, Some("wss://api.huobi.pro/feed"));
            ws_client.subscribe_orderbook(&symbols);
            ws_client.run(duration);
        }
        MarketType::InverseFuture
        | MarketType::LinearSwap
        | MarketType::InverseSwap
        | MarketType::EuropeanOption => crawl_event(
            EXCHANGE_NAME,
            MessageType::L2Event,
            market_type,
            symbols,
            tx,
            duration,
        ),
        _ => panic!("Huobi does NOT have the {} market type", market_type),
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    let tx = create_conversion_thread(
        EXCHANGE_NAME.to_string(),
        MessageType::FundingRate,
        market_type,
        tx,
    );

    let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        vec!["*".to_string()]
    } else {
        symbols.unwrap().to_vec()
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!(r#"{{"topic":"public.{}.funding_rate","op":"sub"}}"#, symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap => {
            let ws_client =
                HuobiInverseSwapWSClient::new(tx, Some("wss://api.hbdm.com/swap-notification"));
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::LinearSwap => {
            let ws_client = HuobiLinearSwapWSClient::new(
                tx,
                Some("wss://api.hbdm.com/linear-swap-notification"),
            );
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Huobi {} does NOT have funding rates", market_type),
    }
}
