use super::{crawl_event, utils::fetch_symbols_retry};
use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_rest_client::*;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "okex";

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    if market_type == MarketType::EuropeanOption
        && (symbols.is_none() || symbols.unwrap().is_empty())
    {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Trade,
            market_type,
            tx,
        );

        let underlying = OkexRestClient::fetch_option_underlying()
            .unwrap_or_else(|_| vec!["BTC-USD".to_string(), "ETH-USD".to_string()]);
        let channels: Vec<String> = underlying
            .into_iter()
            .map(|x| format!("option/trades:{}", x))
            .collect();

        let ws_client = OkexWSClient::new(tx, None);
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
        fetch_symbols_retry(EXCHANGE_NAME, market_type)
    } else {
        symbols.unwrap().to_vec()
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!("swap/funding_rate:{}", symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_client = OkexWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("OKEx {} does NOT have funding rates", market_type),
    }
}
