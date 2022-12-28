use super::{
    crawl_candlestick_ext, crawl_event,
    utils::{check_args, fetch_symbols_retry},
};
use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "bitmex";

async fn crawl_all(msg_type: MessageType, tx: Sender<Message>) {
    let tx = create_conversion_thread(EXCHANGE_NAME.to_string(), msg_type, MarketType::Unknown, tx);

    let channel: &str = match msg_type {
        MessageType::Trade => "trade",
        MessageType::L2Event => "orderBookL2_25",
        MessageType::L2TopK => "orderBook10",
        MessageType::BBO => "quote",
        MessageType::L2Snapshot => "orderBookL2",
        MessageType::FundingRate => "funding",
        _ => panic!("unsupported message type {}", msg_type),
    };
    let commands = vec![format!(r#"{{"op":"subscribe","args":["{}"]}}"#, channel)];

    let ws_client = BitmexWSClient::new(tx, None).await;
    ws_client.send(&commands).await;
    ws_client.run().await;
    ws_client.close();
}

pub(crate) async fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::Trade, tx).await;
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::Trade, market_type, symbols, tx).await;
    }
}

pub(crate) async fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2Event, tx).await;
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::L2Event, market_type, symbols, tx).await;
    }
}

pub(crate) async fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::BBO, tx).await;
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::BBO, market_type, symbols, tx).await;
    }
}

pub(crate) async fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::L2TopK, tx).await;
    } else {
        crawl_event(EXCHANGE_NAME, MessageType::L2TopK, market_type, symbols, tx).await;
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) async fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Unknown {
        // crawl all symbols
        crawl_all(MessageType::FundingRate, tx).await;
    } else {
        let is_empty = match symbols {
            Some(list) => {
                if list.is_empty() {
                    true
                } else {
                    check_args(EXCHANGE_NAME, market_type, list);
                    false
                }
            }
            None => true,
        };

        let real_symbols = if is_empty {
            tokio::task::block_in_place(move || fetch_symbols_retry(EXCHANGE_NAME, market_type))
        } else {
            symbols.unwrap().to_vec()
        };
        if real_symbols.is_empty() {
            panic!("real_symbols is empty");
        }
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::FundingRate,
            market_type,
            tx,
        );

        let topics: Vec<(String, String)> =
            real_symbols.iter().map(|symbol| ("funding".to_string(), symbol.to_string())).collect();

        match market_type {
            MarketType::InverseSwap | MarketType::QuantoSwap => {
                let ws_client = BitmexWSClient::new(tx, None).await;
                ws_client.subscribe(&topics).await;
                ws_client.run().await;
                ws_client.close();
            }
            _ => panic!("BitMEX {} does NOT have funding rates", market_type),
        }
    }
}

pub(crate) async fn crawl_candlestick(
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    tx: Sender<Message>,
) {
    if market_type == MarketType::Unknown {
        let tx = create_conversion_thread(
            EXCHANGE_NAME.to_string(),
            MessageType::Candlestick,
            market_type,
            tx,
        );

        let commands = vec![
            r#"{"op":"subscribe","args":["tradeBin1m"]}"#.to_string(),
            r#"{"op":"subscribe","args":["tradeBin5m"]}"#.to_string(),
        ];

        let ws_client = BitmexWSClient::new(tx, None).await;
        ws_client.send(&commands).await;
        ws_client.run().await;
        ws_client.close();
    } else {
        crawl_candlestick_ext(EXCHANGE_NAME, market_type, symbol_interval_list, tx).await;
    }
}
