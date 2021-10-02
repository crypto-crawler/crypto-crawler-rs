use carbonbot::utils::connect_redis;
use carbonbot::writers::{FileWriter, Writer};
use crypto_crawler::*;
use dashmap::DashMap;
use log::*;
use redis::{self, Commands};
use std::{
    env,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
};

fn to_string(msg: Message) -> Vec<String> {
    if std::env::var("PARSER").is_ok() && std::env::var("PARSER").unwrap() == "true" {
        match msg.msg_type {
            MessageType::Trade => {
                let trades =
                    crypto_msg_parser::parse_trade(&msg.exchange, msg.market_type, &msg.json)
                        .unwrap();
                trades
                    .iter()
                    .map(|x| serde_json::to_string(x).unwrap())
                    .collect()
            }
            MessageType::L2Event => {
                let orderbooks = crypto_msg_parser::parse_l2(
                    &msg.exchange,
                    msg.market_type,
                    &msg.json,
                    Some(msg.received_at as i64),
                )
                .unwrap();
                orderbooks
                    .iter()
                    .map(|x| serde_json::to_string(x).unwrap())
                    .collect()
            }
            MessageType::FundingRate => {
                let rates = crypto_msg_parser::parse_funding_rate(
                    &msg.exchange,
                    msg.market_type,
                    &msg.json,
                )
                .unwrap();
                rates
                    .iter()
                    .map(|x| serde_json::to_string(x).unwrap())
                    .collect()
            }
            _ => panic!("Parser does NOT support {} yet", msg.msg_type),
        }
    } else {
        vec![serde_json::to_string(&msg).unwrap()]
    }
}

pub fn crawl(
    exchange: &'static str,
    market_type: MarketType,
    msg_type: MessageType,
    data_dir: Option<String>,
    redis_url: Option<String>,
) {
    let data_dir_clone = Arc::new(data_dir);
    let writers_map: Arc<DashMap<String, FileWriter>> = Arc::new(DashMap::new());
    let writers_map_clone = writers_map.clone();

    let redis_conn = if let Some(url) = redis_url {
        let conn = match connect_redis(&url) {
            Ok(conn) => Some(conn),
            Err(_) => None,
        };
        Arc::new(Mutex::new(conn))
    } else {
        Arc::new(Mutex::new(None))
    };
    let redis_conn_clone = redis_conn;

    let on_msg_ext = Arc::new(Mutex::new(move |msg: Message| {
        let key = format!("{}-{}-{}", msg_type, exchange, market_type);
        if let Some(ref data_dir) = *data_dir_clone {
            if !writers_map.contains_key(&key) {
                let data_dir = Path::new(data_dir)
                    .join(msg_type.to_string())
                    .join(exchange)
                    .join(market_type.to_string())
                    .into_os_string();
                std::fs::create_dir_all(data_dir.as_os_str()).unwrap();

                let file_name = format!("{}.{}.{}", exchange, market_type, msg_type);
                let file_path = Path::new(data_dir.as_os_str())
                    .join(file_name)
                    .into_os_string();
                writers_map.insert(
                    key.clone(),
                    FileWriter::new(file_path.as_os_str().to_str().unwrap()),
                );
            }
        }

        let string_arr = to_string(msg);
        for s in string_arr.into_iter() {
            if let Some(writer) = writers_map.get(&key) {
                writer.write(&s);
            }

            let mut guard = redis_conn_clone.lock().unwrap();
            if let Some(ref mut conn) = *guard {
                let topic = format!("carbonbot:{}", serde_json::to_string(&msg_type).unwrap());
                if let Err(err) = conn.publish::<&str, String, i64>(&topic, s) {
                    error!("{}", err);
                }
            }
        }
    }));

    let crawl_func = match msg_type {
        MessageType::BBO => crawl_bbo,
        MessageType::Trade => crawl_trade,
        MessageType::L2Event => crawl_l2_event,
        MessageType::L3Event => crawl_l3_event,
        MessageType::L2Snapshot => crawl_l2_snapshot,
        MessageType::L2TopK => crawl_l2_topk,
        MessageType::L3Snapshot => crawl_l3_snapshot,
        MessageType::Ticker => crawl_ticker,
        MessageType::FundingRate => crawl_funding_rate,
        _ => panic!("Not implemented"),
    };
    crawl_func(exchange, market_type, None, on_msg_ext, None);

    for kv in writers_map_clone.iter() {
        let writer = kv.value();
        writer.close();
    }
}

fn main() {
    env_logger::init();

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

    crawl(exchange, market_type, msg_type, data_dir, redis_url);
}
