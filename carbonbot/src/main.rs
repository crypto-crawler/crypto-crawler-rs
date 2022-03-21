use carbonbot::{crawl_other, create_writer_threads};
use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use log::*;
use std::{env, str::FromStr};

pub async fn crawl(
    exchange: &'static str,
    market_type: MarketType,
    msg_type: MessageType,
    data_dir: Option<String>,
    redis_url: Option<String>,
    symbols: Option<&[String]>,
) {
    if data_dir.is_none() && redis_url.is_none() {
        error!("Both DATA_DIR and REDIS_URL are not set");
        return;
    }
    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let writer_threads = create_writer_threads(rx, data_dir, redis_url);

    if msg_type == MessageType::Candlestick {
        crawl_candlestick(exchange, market_type, None, tx).await;
    } else if msg_type == MessageType::OpenInterest {
        tokio::task::spawn_blocking(move || crawl_open_interest(exchange, market_type, tx));
    } else if msg_type == MessageType::Other {
        crawl_other(exchange, market_type, tx).await;
    } else {
        match msg_type {
            MessageType::BBO => {
                crawl_bbo(exchange, market_type, symbols, tx).await;
            }
            MessageType::Trade => {
                crawl_trade(exchange, market_type, symbols, tx).await;
            }
            MessageType::L2Event => {
                crawl_l2_event(exchange, market_type, symbols, tx).await;
            }
            MessageType::L3Event => {
                crawl_l3_event(exchange, market_type, symbols, tx).await;
            }
            MessageType::L2Snapshot => {
                let symbols = if let Some(symbols) = symbols {
                    symbols.to_vec()
                } else {
                    vec![]
                };
                tokio::task::spawn_blocking(move || {
                    let symbols_local = symbols;
                    crawl_l2_snapshot(exchange, market_type, Some(&symbols_local), tx)
                });
            }
            MessageType::L2TopK => {
                crawl_l2_topk(exchange, market_type, symbols, tx).await;
            }
            MessageType::L3Snapshot => {
                let symbols = if let Some(symbols) = symbols {
                    symbols.to_vec()
                } else {
                    vec![]
                };
                tokio::task::spawn_blocking(move || {
                    let symbols_local = symbols;
                    crawl_l3_snapshot(exchange, market_type, Some(&symbols_local), tx)
                });
            }
            MessageType::Ticker => {
                crawl_ticker(exchange, market_type, symbols, tx).await;
            }
            MessageType::FundingRate => {
                crawl_funding_rate(exchange, market_type, symbols, tx).await
            }
            _ => panic!("Not implemented"),
        };
    }
    for thread in writer_threads {
        thread.join().unwrap();
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 && args.len() != 5 {
        println!("Usage: carbonbot <exchange> <market_type> <msg_type> [comma_seperated_symbols]");
        return;
    }

    let exchange: &'static str = Box::leak(args[1].clone().into_boxed_str());

    let market_type = MarketType::from_str(&args[2]);
    if market_type.is_err() {
        println!("Unknown market type: {}", &args[2]);
        return;
    }
    let market_type = market_type.unwrap();

    let msg_type = MessageType::from_str(&args[3]);
    if msg_type.is_err() {
        println!("Unknown msg type: {}", &args[3]);
        return;
    }
    let msg_type = msg_type.unwrap();

    let data_dir = if std::env::var("DATA_DIR").is_err() {
        info!("The DATA_DIR environment variable does not exist");
        None
    } else {
        let url = std::env::var("DATA_DIR").unwrap();
        Some(url)
    };

    let redis_url = if std::env::var("REDIS_URL").is_err() {
        info!("The REDIS_URL environment variable does not exist");
        None
    } else {
        let url = std::env::var("REDIS_URL").unwrap();
        Some(url)
    };

    let specified_symbols = if args.len() == 4 {
        Vec::new()
    } else {
        let mut symbols = fetch_symbols_retry(exchange, market_type);
        symbols.retain(|symbol| args[4].split(',').any(|part| symbol.contains(part)));
        info!("target symbols: {:?}", symbols);
        symbols
    };

    if data_dir.is_none() && redis_url.is_none() {
        panic!("The environment variable DATA_DIR and REDIS_URL are not set, at least one of them should be set");
    }

    crawl(
        exchange,
        market_type,
        msg_type,
        data_dir,
        redis_url,
        if specified_symbols.is_empty() {
            None
        } else {
            Some(&specified_symbols)
        },
    )
    .await;
}
