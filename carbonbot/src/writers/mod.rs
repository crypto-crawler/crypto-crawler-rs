pub(super) mod file_writer;

use crate::utils::connect_redis;
use crypto_crawler::*;
use log::*;
use redis::{self, Commands};
use std::collections::HashMap;
use std::thread::JoinHandle;
use std::{
    path::Path,
    sync::mpsc::{Receiver, Sender},
};

pub trait Writer {
    fn write(&mut self, s: &str);
    fn close(&mut self);
}

pub use file_writer::FileWriter;

fn create_file_writer_thread(
    rx: Receiver<Message>,
    data_dir: String,
    tx_redis: Option<Sender<Message>>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut writers: HashMap<String, FileWriter> = HashMap::new();
        for msg in rx {
            let file_name = format!("{}.{}.{}", msg.exchange, msg.market_type, msg.msg_type);
            if !writers.contains_key(&file_name) {
                let data_dir = Path::new(&data_dir)
                    .join(msg.msg_type.to_string())
                    .join(&msg.exchange)
                    .join(msg.market_type.to_string())
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

            let s = serde_json::to_string(&msg).unwrap();

            if let Some(writer) = writers.get_mut(&file_name) {
                writer.write(&s);
            }
            // copy to redis
            if let Some(ref tx_redis) = tx_redis {
                tx_redis.send(msg).unwrap();
            }
        }
        for mut writer in writers {
            writer.1.close();
        }
    })
}

fn create_redis_writer_thread(rx: Receiver<Message>, redis_url: String) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut redis_conn = connect_redis(&redis_url).unwrap();
        for msg in rx {
            let msg_type = msg.msg_type;
            let s = serde_json::to_string(&msg).unwrap();
            let topic = format!("carbonbot:{}", msg_type);
            if let Err(err) = redis_conn.publish::<&str, String, i64>(&topic, s) {
                error!("{}", err);
            }
        }
    })
}

pub fn create_writer_threads(
    rx: Receiver<Message>,
    data_dir: Option<String>,
    redis_url: Option<String>,
) -> Vec<JoinHandle<()>> {
    let mut threads = Vec::new();
    // channel for Redis
    let (tx_redis, rx_redis) = std::sync::mpsc::channel::<Message>();
    if let Some(data_dir) = data_dir {
        let thread = if redis_url.is_none() {
            create_file_writer_thread(rx, data_dir, None)
        } else {
            create_file_writer_thread(rx, data_dir, Some(tx_redis))
        };
        threads.push(thread);
    }
    if let Some(redis_url) = redis_url {
        let thread = create_redis_writer_thread(rx_redis, redis_url);
        threads.push(thread);
    }
    threads
}
