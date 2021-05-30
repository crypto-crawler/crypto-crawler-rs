use std::{
    collections::HashMap,
    env,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
};

use carbonbot::{
    utils::connect_redis,
    writers::{FileWriter, Writer},
};
use crypto_crawler::MarketType;
use crypto_ws_client::*;
use dashmap::DashMap;
use log::*;
use redis::{self, Commands};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: carbonbot <market_type>");
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

    let data_dir_clone = Arc::new(data_dir);

    let writers_map: Arc<DashMap<String, FileWriter>> = Arc::new(DashMap::new());

    let redis_conn = if let Some(url) = redis_url {
        let conn = match connect_redis(&url) {
            Ok(conn) => Some(conn),
            Err(_) => None,
        };
        Arc::new(Mutex::new(conn))
    } else {
        Arc::new(Mutex::new(None))
    };

    let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
        let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();
        let ch = obj.get("ch").unwrap().as_str().unwrap();
        let key = format!("{}-{}", market_type, ch);
        if let Some(ref data_dir) = *data_dir_clone {
            if !writers_map.contains_key(&key) {
                let data_dir = Path::new(data_dir)
                    .join("misc")
                    .join("huobi")
                    .join(market_type.to_string())
                    .join(ch)
                    .into_os_string();
                std::fs::create_dir_all(data_dir.as_os_str()).unwrap();

                let file_name = format!("huobi.{}.{}", market_type, ch);
                let file_path = Path::new(data_dir.as_os_str())
                    .join(file_name)
                    .into_os_string();
                writers_map.insert(
                    key.clone(),
                    FileWriter::new(file_path.as_os_str().to_str().unwrap()),
                );
            }
        }

        if let Some(writer) = writers_map.get(&key) {
            writer.write(&msg);
        }

        let mut guard = redis_conn.lock().unwrap();
        if let Some(ref mut conn) = *guard {
            if let Err(err) = conn.publish::<&str, String, i64>("carbonbot:misc:huobi", msg) {
                error!("{}", err);
            }
        }
    }));

    let channels: Vec<String> = vec!["market.overview".to_string()];

    match market_type {
        MarketType::Spot => {
            let ws_client = HuobiSpotWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        MarketType::InverseFuture => {
            let ws_client = HuobiFutureWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        MarketType::LinearSwap => {
            let ws_client = HuobiLinearSwapWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        MarketType::InverseSwap => {
            let ws_client = HuobiInverseSwapWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        MarketType::EuropeanOption => {
            let ws_client = HuobiOptionWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(None);
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}
