use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use carbonbot::{
    utils::connect_redis,
    writers::{FileWriter, Writer},
};
use crypto_ws_client::{BitmexWSClient, WSClient};
use dashmap::DashMap;
use log::*;
use redis::{self, Commands};
use serde_json::Value;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
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
        let table = obj.get("table").unwrap().as_str().unwrap();
        let key = format!("bitmex-{}", table);
        if let Some(ref data_dir) = *data_dir_clone {
            if !writers_map.contains_key(&key) {
                let data_dir = Path::new(data_dir)
                    .join("misc")
                    .join("bitmex")
                    .join(table)
                    .into_os_string();
                std::fs::create_dir_all(data_dir.as_os_str()).unwrap();

                let file_name = format!("bitmex.misc.{}", table);
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
            if let Err(err) = conn.publish::<&str, String, i64>("carbonbot:misc:bitmex", msg) {
                error!("{}", err);
            }
        }
    }));

    let ws_client = BitmexWSClient::new(on_msg_ext, None);
    let channels: Vec<String> = vec![
        "announcement",
        "connected",
        "instrument",
        "liquidation",
        "publicNotifications",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();
    ws_client.subscribe(&channels);
    ws_client.run(None);
}
