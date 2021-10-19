use std::{
    collections::HashMap, env, path::Path, str::FromStr, sync::mpsc::Receiver, thread::JoinHandle,
};

use carbonbot::{
    utils::connect_redis,
    writers::{FileWriter, Writer},
};
use crypto_crawler::MarketType;
use crypto_ws_client::*;
use log::*;
use redis::{self, Commands};
use serde_json::Value;

fn create_writer_thread(
    market_type: MarketType,
    rx: Receiver<String>,
    data_dir: Option<String>,
    redis_url: Option<String>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut writers: HashMap<String, FileWriter> = HashMap::new();
        let mut redis_conn = if let Some(url) = redis_url {
            match connect_redis(&url) {
                Ok(conn) => Some(conn),
                Err(_) => None,
            }
        } else {
            None
        };
        for msg in rx {
            let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();
            let ch = obj
                .get("data")
                .unwrap()
                .as_object()
                .unwrap()
                .get("e")
                .unwrap()
                .as_str()
                .unwrap();
            let file_name = format!("binance.{}.{}", market_type, ch);
            if let Some(ref data_dir) = data_dir {
                if !writers.contains_key(&file_name) {
                    let data_dir = Path::new(data_dir)
                        .join("misc")
                        .join("binance")
                        .join(market_type.to_string())
                        .join(ch)
                        .into_os_string();
                    std::fs::create_dir_all(data_dir.as_os_str()).unwrap();

                    let file_path = Path::new(data_dir.as_os_str())
                        .join(file_name.clone())
                        .into_os_string();
                    writers.insert(
                        file_name.clone(),
                        FileWriter::new(file_path.as_os_str().to_str().unwrap()),
                    );
                }
            }

            if let Some(writer) = writers.get_mut(&file_name) {
                writer.write(&msg);
            }

            if let Some(ref mut conn) = redis_conn {
                if let Err(err) = conn.publish::<&str, String, i64>("carbonbot:misc:binance", msg) {
                    error!("{}", err);
                }
            }
        }
    })
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: binance_misc <market_type>");
        return;
    }
    let market_type = MarketType::from_str(&args[1]);
    if market_type.is_err() {
        println!("Unknown market type: {}", &args[1]);
        return;
    }
    let market_type = market_type.unwrap();

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

    if data_dir.is_none() && redis_url.is_none() {
        panic!("The environment variable DATA_DIR and REDIS_URL are not set, at least one of them should be set");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let writer_thread = create_writer_thread(market_type, rx, data_dir, redis_url);

    let channels: Vec<String> = vec!["!forceOrder@arr".to_string()];

    match market_type {
        MarketType::InverseSwap => {
            let ws_client = BinanceInverseWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        MarketType::LinearSwap => {
            let ws_client = BinanceLinearWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
    writer_thread.join().unwrap();
}
