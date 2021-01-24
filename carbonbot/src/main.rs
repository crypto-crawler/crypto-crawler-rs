mod writers;

use crypto_crawler::*;
use std::{
    env,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
};
use writers::{RotatedFileWriter, Writer};

use simplelog::*;

use dashmap::{DashMap, DashSet};
use std::fs::File;

pub fn crawl(
    exchange: &'static str,
    market_type: MarketType,
    msg_type: MessageType,
    data_dir: &'static str,
) {
    let writers_map: Arc<DashMap<String, RotatedFileWriter>> = Arc::new(DashMap::new());
    let symbols: Arc<DashSet<String>> = Arc::new(DashSet::new());

    let writers_map_clone = writers_map.clone();
    let symbols_clone = symbols.clone();

    let on_msg_ext = Arc::new(Mutex::new(move |msg: Message| {
        let key = format!("{}-{}-{}", msg_type, exchange, market_type);
        symbols_clone.insert(msg.symbol.clone());
        if !writers_map_clone.contains_key(&key) {
            let data_dir = Path::new(data_dir)
                .join(msg_type.to_string())
                .join(exchange)
                .join(market_type.to_string())
                .into_os_string();
            let prefix = format!("{}.{}.{}.", exchange, market_type, msg_type);
            writers_map_clone.insert(
                key.clone(),
                RotatedFileWriter::new(data_dir.as_os_str().to_str().unwrap(), prefix.as_str()),
            );
        }

        let writer = writers_map_clone.get(&key).unwrap();
        let json = serde_json::to_string(&msg).unwrap();
        writer.write(&json);
    }));

    match msg_type {
        MessageType::Trade => crawl_trade(exchange, market_type, None, on_msg_ext, None),
        MessageType::L2Event => crawl_l2_event(exchange, market_type, None, on_msg_ext, None),
        MessageType::L3Event => crawl_l3_event(exchange, market_type, None, on_msg_ext, None),
        MessageType::L2Snapshot => {
            crawl_l2_snapshot(exchange, market_type, None, on_msg_ext, None, None)
        }
        MessageType::L3Snapshot => {
            crawl_l3_snapshot(exchange, market_type, None, on_msg_ext, None, None)
        }
        _ => panic!("Not implemented"),
    }

    for symbol in symbols.iter() {
        println!("{}", *symbol);
    }

    for kv in writers_map.iter() {
        let writer = kv.value();
        writer.close();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: carbonbot <exchange> <market_type> <msg_type>");
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

    if std::env::var("DATA_DIR").is_err() {
        panic!("Please set the DATA_DIR environment variable");
    }
    let data_dir: &'static str = Box::leak(std::env::var("DATA_DIR").unwrap().into_boxed_str());

    let _ = std::fs::create_dir_all(Path::new(data_dir).join("logs"));
    let _ = WriteLogger::init(
        LevelFilter::Warn,
        Config::default(),
        File::create(
            Path::new(data_dir)
                .join("logs")
                .join(format!("{}-{}-{}.log", exchange, market_type, msg_type)),
        )
        .unwrap(),
    );

    crawl(exchange, market_type, msg_type, data_dir);
}
