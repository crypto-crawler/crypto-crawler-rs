use chrono::Utc;
use crypto_crawler::{
    crawl_bbo, crawl_candlestick, crawl_funding_rate, crawl_l2_event, crawl_l2_topk, MarketType,
    Message, MessageType, crawl_trade,
};
use crypto_msg_parser::{parse_l2_topk, Order, OrderBookMsg, TradeMsg, TradeSide, parse_trade, parse_bbo, BboMsg, KlineMsg, parse_candlestick};
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use yata::helpers::{MA, RandomCandles};
use yata::indicators::MACD;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::SystemTime;

use yata::prelude::*;
use yata::methods::EMA;

mod file_save;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref EXANGE: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("crypto", 1);
        m.insert("ftx", 2);
        m.insert("binance", 3);
        m.insert("okx", 11);
        m
    };
    static ref SYMBLE: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("BTC/USDT", 1);
        m.insert("BTC/USD", 2);
        m.insert("USDT/USD", 3);
        m
    };
    static ref INFOTYPE: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("asks", 1);
        m.insert("bids", 2);
        m
    };
    static ref PERIOD: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("1m", 1);
        m.insert("5m", 2);
        m.insert("30m", 3);
        m.insert("1h", 4);
        m
    };
}

pub fn long_to_hex(num: i64) -> String {
    let num_hex = format!("{:x}", num); // to hex
    let mut num_hex_len = num_hex.len() / 2;
    if (num_hex_len * 2 < num_hex.len()) {
        num_hex_len = (num_hex_len + 1);
    }
    let pad_len = num_hex_len * 2;
    let long_hex = format!("{0:0>pad_len$}", num_hex, pad_len = pad_len);
    long_hex
}

fn hex_to_byte(mut hex: String) -> Vec<i8> {
    hex = str::replace(&hex, " ", "");
    let mut bytes: Vec<i8> = Vec::new();

    if hex.len() % 2 == 1 {
        return bytes;
    }

    let mut hex_split: Vec<String> = Vec::new();
    for i in 0..(hex.len() / 2) {
        let str = &hex[i * 2..i * 2 + 2];
        hex_split.push(str.to_string());
    }

    for i in hex_split.iter() {
        let num = i32::from_str_radix(i, 16);
        let i8_num = num.unwrap() as i8;
        bytes.push(i8_num);
        // match num {
        //     Ok(t) => bytes.push(t),
        //     Err(_err) => break
        // }
    }

    bytes
}

fn encode_num_to_bytes(mut value: String) -> [i8; 5] {
    let mut result: [i8; 5] = [0; 5];
    let mut e = 0;

    // if value.find("E-") != Some(0) {
    //     let split: Vec<&str> = value.split("E-").collect();
    //     let a = split[1];
    //     e = a.parse().unwrap();
    //     value = split[0].to_string();
    // }

    result[4] = match value.find(".") {
        Some(_index) => value.len() - _index - 1 + e,
        None => 0,
    } as i8;

    value = value.replace(".", "");
    let hex_str = long_to_hex(value.parse().unwrap());
    let hex_byte = hex_to_byte(hex_str);
    let length = hex_byte.len();
    if hex_byte.len() > 0 {
        result[3] = *hex_byte.get(length - 1).unwrap();
        if hex_byte.len() > 1 {
            result[2] = *hex_byte.get(length - 2).unwrap();
            if hex_byte.len() > 2 {
                result[1] = *hex_byte.get(length - 3).unwrap();
                if hex_byte.len() > 3 {
                    result[0] = *hex_byte.get(length - 4).unwrap();
                }
            }
        }
    }

    result
}

fn roundtrip(data: &[u8]) {
    // Compress the input
    let compressed = compress_to_vec(data, 6);
    println!("compressed from {} to {}", data.len(), compressed.len());
    // Decompress the compressed input
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
    // Check roundtrip succeeded
    assert_eq!(data, decompressed);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    let mut candles = RandomCandles::new();
    let mut macd = MACD::default();

    // macd.method1 = "sma-4".parse().unwrap(); // one way of defining methods inside indicators

    macd.signal = MA::TEMA(5); // another way of defining methods inside indicators

    let mut macd = macd.init(&candles.first()).unwrap();

    for candle in candles.take(10) {
        let result = macd.next(&candle);

        println!("{:?}", result);
    }

    // let num = u8::from_str_radix("9a", 16);

    // let i32_num = i32::from_str_radix("9a", 16);
    // let i8_num = i32_num.unwrap() as i8;

    // let test = encode_num_to_bytes("10000000.99".to_string());
    // println!("2");

    // EMA of length=3
    let mut ema = EMA::new(3, &3.0).unwrap();

    println!("{:#}", ema.next(&3.0));
    println!("{:#}", ema.next(&6.0));

    assert_eq!(ema.next(&9.0), 6.75);
    assert_eq!(ema.next(&12.0), 9.375);

    let mut write_data_object = file_save::WriteData::new();
    let result = write_data_object.start();

    // let mut old: Option<&OrderBookMsg> = None;

    let (tx, rx) = std::sync::mpsc::channel();
    println!("1");
    tokio::task::spawn(async move {
        let mut o: Option<Arc<OrderBookMsg>> = None;

        for msg in rx {

            let msg: Message = msg;
            let received_at = Utc::now().timestamp_millis();
            println!("{:#}", msg);

            // let trade = &parse_trade("dydx", MarketType::LinearSwap, &msg.json).unwrap()[0];
            // let raw_msg = r#"{"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1622363903670,"s":"BTCUSDT","U":11294093710,"u":11294093726,"b":[["35743.98000000","0.00000000"],["35743.87000000","0.00001500"]],"a":[["35743.88000000","0.24000000"],["35743.97000000","0.00000000"]]}}"#;
            // let orderbook = &parse_l2("binance", MarketType::Spot, raw_msg, None).unwrap()[0];
            // let orderbook = &parse_l2("binance", MarketType::Spot, &(msg as Message).json, None).unwrap()[0];

            // parse
            // let received_at = msg.received_at;
            // let orderbook = Arc::new(tokio::task::spawn_blocking(move || {
            //     parse_l2_topk(
            //         "okx",
            //         MarketType::Spot,
            //         &(msg as Message).json,
            //         //Some(received_at),
            //         None,
            //     )
            //     .unwrap()
            // })
            // .await
            // .ok()
            // .and_then(|v| v.into_iter().next())
            // .unwrap());

            // let trade = Arc::new(tokio::task::spawn_blocking(move || {
            //     parse_trade(
            //         "okx",
            //         MarketType::Spot,
            //         &(msg as Message).json,
            //     )
            //     .unwrap()
            // })
            // .await
            // .ok()
            // .and_then(|v| v.into_iter().next())
            // .unwrap());

            // let bbo = Arc::new(tokio::task::spawn_blocking(move || {
            //     parse_bbo(
            //         "huobi",
            //         MarketType::Spot,
            //         &(msg as Message).json,
            //         Some(received_at),
            //     )
            //     .unwrap()
            // })
            // .await
            // .ok()
            // .unwrap());

            let kline = Arc::new(tokio::task::spawn_blocking(move || {
                parse_candlestick(
                    "binance",
                    MarketType::Spot,
                    &(msg as Message).json,
                    MessageType::Candlestick,
                )
                .unwrap()
            })
            .await
            .ok()
            .unwrap());

            // encode
            // let orderbook_for_encode = orderbook.clone();
            // let order_book_bytes =
            //     Arc::new(tokio::task::spawn_blocking(move || encode_orderbook(&orderbook_for_encode))
            //         .await
            //         .unwrap());

            // let trade_for_encode = trade.clone();
            // let trade_bytes =
            //     Arc::new(tokio::task::spawn_blocking(move || encode_trade(&trade_for_encode))
            //         .await
            //         .unwrap());

            // let bbo_for_encode = bbo.clone();
            // let bbo_bytes =
            //     Arc::new(tokio::task::spawn_blocking(move || encode_bbo(&bbo_for_encode))
            //         .await
            //         .unwrap());

            // decode
            // let order_book_bytes_decode = order_book_bytes.clone();
            // let order_book_decode = 
            //     tokio::task::spawn_blocking(move || decode_orderbook(order_book_bytes_decode.to_vec()))
            //     .await
            //     .unwrap();

            // let trade_bytes_decode = trade_bytes.clone();
            // let trade_decode = 
            //     tokio::task::spawn_blocking(move || decode_trade(trade_bytes_decode.to_vec()))
            //     .await
            //     .unwrap();

            // let bbo_bytes_decode = bbo_bytes.clone();
            // let bbo_decode = 
            //     tokio::task::spawn_blocking(move || decode_bbo(bbo_bytes_decode.to_vec()))
            //     .await
            //     .unwrap();


            // caculate diff    
            // if let Some(ref old) = o {
            //     let diff = generated_diffs(old, &orderbook);
            //     println!("2");
            // }

            // o = Some(orderbook.clone());

            // let order_book_bytes_save = order_book_bytes.clone();
            // write_data_object.add_order_book(
            //     "name".to_string(),
            //     "ydf".to_string(),
            //     order_book_bytes_save.to_vec(),
            // );

            println!("finished");
        }
    });

    let symbols_vec = &vec!["btcusdt".to_string()];
    let symbols_opt: Option<&[String]> = Some(symbols_vec);

    // Crawl realtime trades for all symbols of binance inverse_swap markets
    // crawl_trade("okx", MarketType::Spot, symbols_opt, tx).await;
    // crawl_l2_event("huobi", MarketType::Spot, symbols_opt, tx).await;
    // crawl_l2_topk("okx", MarketType::Spot, symbols_opt, tx).await;
    // crawl_bbo("huobi", MarketType::Spot, symbols_opt, tx).await;
    
    let symbol_interval_list: &[(String, usize)] = &vec![("BTCUSDT".to_string(), 60usize)];
    let symbol_interval_list = Some(symbol_interval_list);
    crawl_candlestick("binance", MarketType::Spot, symbol_interval_list, tx).await;

    // crawl_funding_rate("binance", MarketType::InverseSwap, None, tx).await;

    // to intToHex
    // let aa = format!("{:x}", 4133428);
    // let z = i64::from_str_radix("3f", 16);
    // println!("{:?}", z);
    // let mut info_len = u16::from_be_bytes(bb);
    // let bb = aa.as_bytes();

    // let input="1650499221926264889 HUOBI BTCUSDT {\"asks\":[[41334.28,1.447888],[41334.79,0.504350],[41334.8,0.056761],[41335.4,0.01],[41336.04,0.4],[41336.54,0.612022],[41336.85,0.121715],[41337.17,0.060147],[41337.51,0.107953],[41337.52,0.040467],[41338.76,0.63319],[41339.79,0.031891],[41339.8,0.005544],[41340.73,0.036],[41341.39,0.4],[41341.62,0.300589],[41342.05,0.001384],[41342.26,0.05],[41344.62,0.012073],[41344.94,0.4]],\"bids\":[[41334.27,0.005322],[41329.95,0.437311],[41328.38,0.086845],[41328.37,0.011155],[41327.6,4.67E-4],[41327.38,0.005],[41327.37,0.043911],[41326.83,0.005],[41326.82,0.006381],[41323.11,0.004173],[41322.23,0.090619],[41321.84,0.2],[41321.32,0.32],[41319.43,0.165819],[41316.71,0.11362],[41315.88,0.008347],[41314.66,0.026265],[41314.56,0.006405],[41311.78,0.001598],[41310.96,0.087]]}";
    // let input2="1650499221926264889 HUOBI BTCUSDT {\"asks\":[[41334.28,11.447888],[41334.8,0.056761],[41335.4,0.01],[41336.04,0.4],[41336.539,0.612022],[41336.54,0.612022],[41336.85,0.121715],[41337.17,0.060147],[41337.51,0.107953],[41337.52,0.040467],[41338.76,0.63319],[41339.79,0.031891],[41339.8,0.005544],[41340.73,0.036],[41341.39,0.4],[41341.62,0.300589],[41342.05,0.001384],[41342.26,0.05],[41344.62,0.012073],[41344.94,0.4],[41344.95,0.4]],\"bids\":[[41334.27,0.005322],[41329.95,0.437311],[41328.38,0.086845],[41328.37,0.011155],[41327.6,4.67E-4],[41327.38,0.005],[41327.37,0.043911],[41326.83,0.005],[41326.82,0.006381],[41323.11,0.004173],[41322.23,0.090619],[41321.84,0.2],[41321.32,0.32],[41319.43,0.165819],[41316.71,0.11362],[41315.88,0.008347],[41314.66,0.026265],[41314.56,0.006405],[41311.78,0.001598],[41310.96,0.087]]}";
    // let data = input.split(" ").collect::<Vec<&str>>();
    // let marks = data[data.len() - 1];
    // let new = getOrderBook("ho".to_string(), marks.to_string())
    //     .unwrap()
    //     .unwrap();
    // let data = input2.split(" ").collect::<Vec<&str>>();
    // let marks = data[data.len() - 1];
    // let old = getOrderBook("ho".to_string(), marks.to_string())
    //     .unwrap()
    //     .unwrap();
    // let diff_oreder_book= old.generated_diffs(new);
    // let restore_oreder_book = old.restore_data(diff_oreder_book);
}


pub fn generated_diffs(old: &OrderBookMsg, latest: &OrderBookMsg) -> OrderBookMsg {
    let mut diff = OrderBookMsg {
        asks: vec![],
        bids: vec![],
        exchange: latest.exchange.clone(),
        market_type: latest.market_type.clone(),
        symbol: latest.symbol.clone(),
        pair: latest.pair.clone(),
        msg_type: latest.msg_type.clone(),
        timestamp: latest.timestamp,
        snapshot: latest.snapshot,
        seq_id: latest.seq_id,
        prev_seq_id: latest.prev_seq_id,
        json: latest.json.clone(),
    };
    diff.asks = get_orders(&old.asks, &latest.asks, OrderType::Ask);
    diff.bids = get_orders(&old.bids, &latest.bids, OrderType::Bid);
    diff
}


pub fn restore_diffs(old: &OrderBookMsg, diff: &OrderBookMsg) -> OrderBookMsg {
    let mut diff = OrderBookMsg {
        asks: vec![],
        bids: vec![],
        exchange: diff.exchange.clone(),
        market_type: diff.market_type.clone(),
        symbol: diff.symbol.clone(),
        pair: diff.pair.clone(),
        msg_type: diff.msg_type.clone(),
        timestamp: diff.timestamp,
        snapshot: diff.snapshot,
        seq_id: diff.seq_id,
        prev_seq_id: diff.prev_seq_id,
        json: diff.json.clone(),
    };
    diff.asks = restore_orders(&old.asks, &diff.asks, OrderType::Ask);
    diff.bids = restore_orders(&old.bids, &diff.bids, OrderType::Bid);
    diff
}

#[derive(PartialEq)]
pub enum OrderType {
    Ask,
    Bid,
}


pub fn get_orders<'a>(new: &'a Vec<Order>, old: &'a Vec<Order>, _type: OrderType) -> Vec<Order> {
    let mut new_index: usize = 0;
    let mut old_index: usize = 0;

    let mut result = Vec::new();
    let mut is_new_remaining = new_index < new.len();
    let mut is_old_remaining = old_index < old.len();
    while is_new_remaining && is_old_remaining {
        let latest_order = &new[new_index];
        let old_order = &old[old_index];
        if latest_order.price == old_order.price
            && latest_order.quantity_quote == old_order.quantity_quote
        {
            old_index += 1;
            new_index += 1;
        } else {
            match (
                latest_order.price == old_order.price,
                latest_order.quantity_quote == old_order.quantity_quote,
            ) {
                (true, false) => {
                    let updated = Order {
                        price: old_order.price,
                        quantity_base: latest_order.quantity_base,
                        quantity_quote: latest_order.quantity_quote,
                        quantity_contract: latest_order.quantity_contract,
                    };
                    result.push(updated);
                    old_index += 1;
                    new_index += 1;
                }
                (false, false) => {
                    let mut cross_over = latest_order.price < old_order.price;
                    if _type == OrderType::Bid {
                        cross_over = latest_order.price > old_order.price;
                    };
                    match cross_over {
                        true => {
                            let removed = Order {
                                price: old_order.price,
                                quantity_base: 0.0,
                                quantity_quote: 0.0,
                                quantity_contract: Some(0.0),
                            };
                            result.push(removed);
                            old_index += 1;
                        }
                        false => {
                            let added = Order {
                                price: old_order.price,
                                quantity_base: 0.0,
                                quantity_quote: 0.0,
                                quantity_contract: Some(0.0),
                            };
                            result.push(added);
                            new_index += 1;
                        }
                    }
                }
                (_, _) => {}
            }
        }
        is_new_remaining = new_index < new.len();
        is_old_remaining = old_index < old.len();
    }
    if is_new_remaining {
        for i in new_index..new.len() {
            let order = &new[i];
            let added = Order {
                price: order.price,
                quantity_base: order.quantity_base,
                quantity_quote: order.quantity_quote,
                quantity_contract: order.quantity_contract,
            };
            result.push(added);
        }
    } else if is_old_remaining {
        for i in old_index..old.len() {
            let order = &old[i];
            let removed = Order {
                price: order.price,
                quantity_base: 0.0,
                quantity_quote: 0.0,
                quantity_contract: None,
            };
            result.push(removed);
        }
    }
    result
}


pub fn restore_orders<'a>(
    old: &'a Vec<Order>,
    diff: &'a Vec<Order>,
    _type: OrderType,
) -> Vec<Order> {
    let mut result = Vec::new();
    if diff.len() == 0 {
        return result;
    }
    let mut diff_index: usize = 0;
    old.iter().for_each(|order| {
        let diff_order = &diff[diff_index];
        let mut is_coss_over = order.price > diff_order.price;
        if _type == OrderType::Bid {
            is_coss_over = order.price < diff_order.price;
        };
        match (
            order.price == diff_order.price,
            is_coss_over,
            diff_order.quantity_quote,
        ) {
            (true, true, 0.0) => {}
            (true, _, _) => {
                let updated = Order {
                    price: diff_order.price,
                    quantity_base: diff_order.quantity_base,
                    quantity_quote: diff_order.quantity_quote,
                    quantity_contract: diff_order.quantity_contract,
                };
                result.push(updated);
            }
            (false, true, _) => {
                let old = Order {
                    price: diff_order.price,
                    quantity_base: diff_order.quantity_base,
                    quantity_quote: diff_order.quantity_quote,
                    quantity_contract: diff_order.quantity_contract,
                };
                result.push(old);
            }
            (false, false, _) => {
                let added = Order {
                    price: order.price,
                    quantity_base: order.quantity_base,
                    quantity_quote: order.quantity_quote,
                    quantity_contract: order.quantity_contract,
                };
                let old = Order {
                    price: diff_order.price,
                    quantity_base: diff_order.quantity_base,
                    quantity_quote: diff_order.quantity_quote,
                    quantity_contract: diff_order.quantity_contract,
                };
                diff_index += 1;
                result.push(added);
                result.push(old);
            }
        }
    });
    // dbg!(&result);
    if diff_index < diff.len() {
        for i in diff_index..diff.len() {
            let order = &diff[i];
            let added = Order {
                price: order.price,
                quantity_base: order.quantity_base,
                quantity_quote: order.quantity_quote,
                quantity_contract: order.quantity_contract,
            };
            result.push(added);
        }
    }
    result
}


pub fn encode_orderbook(orderbook: &OrderBookMsg) -> Vec<i8> {
    let mut orderbook_bytes: Vec<i8> = Vec::new();

    let exchange_timestamp = orderbook.timestamp;

    //1、交易所时间戳:6 or 8 字节时间戳
    let exchange_timestamp_hex = long_to_hex(exchange_timestamp);
    let exchange_timestamp_hex_byte = hex_to_byte(exchange_timestamp_hex);
    orderbook_bytes.extend_from_slice(&exchange_timestamp_hex_byte);

    //2、收到时间戳:6 or 8 字节时间戳
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let now_ms = now.as_millis();
    let received_timestamp_hex = long_to_hex(now_ms as i64);
    let received_timestamp_hex_byte = hex_to_byte(received_timestamp_hex);
    orderbook_bytes.extend_from_slice(&received_timestamp_hex_byte);

    //3、EXANGE 1字节信息标识
    let _exchange = *EXANGE.get(&orderbook.exchange.as_str()).unwrap() as i8;
    orderbook_bytes.push(_exchange);

    //4、MARKET_TYPE 1字节信息标识
    let _market_type = match orderbook.market_type {
        MarketType::Unknown => 0,
        MarketType::Spot => 1,
        MarketType::LinearFuture => 2,
        MarketType::InverseFuture => 3,
        MarketType::LinearSwap => 4,
        MarketType::InverseSwap => 5,
        MarketType::EuropeanOption => 6,
        MarketType::QuantoFuture => 7,
        MarketType::QuantoSwap => 8,
        MarketType::Move => 0,        
        MarketType::BVOL => 0,        
        MarketType::AmericanOption => 0,
    };
    orderbook_bytes.push(_market_type);

    //5、MESSAGE_TYPE 1字节信息标识
    let _message_type = match orderbook.msg_type {
        MessageType::Other => 0,
        MessageType::Trade => 1,
        MessageType::BBO => 2,
        MessageType::L2TopK => 3,
        MessageType::L2Snapshot => 4,
        MessageType::L2Event => 5,
        MessageType::L3Snapshot => 6,
        MessageType::L3Event => 7,
        MessageType::Ticker => 8,
        MessageType::Candlestick => 9,
        MessageType::OpenInterest => 10,
        MessageType::FundingRate => 11,
        MessageType::LongShortRatio => 12,
        MessageType::TakerVolume => 12,
    };
    orderbook_bytes.push(_message_type);

    //6、SYMBLE 2字节信息标识
    let _pair = SYMBLE.get(&orderbook.pair.as_str()).unwrap();
    let _pair_hex = long_to_hex(*_pair as i64);

    let _pair_hex = format!("{:0>4}", _pair_hex);
    let _pair_hex_byte = hex_to_byte(_pair_hex);
    orderbook_bytes.extend_from_slice(&_pair_hex_byte);

    //7、ask、bid
    let mut markets = HashMap::new();
    markets.insert("asks", &orderbook.asks);
    markets.insert("bids", &orderbook.bids);

    for (k, order_list) in markets {
        let _type = (*INFOTYPE.get(k).unwrap()) as i8;
        //1）字节信息标识
        orderbook_bytes.push(_type);

        //2）字节信息体的长度
        let list_len = (order_list.len() * 10) as i64;
        let list_len_hex = long_to_hex(list_len);
        let list_len_hex = format!("{:0>4}", list_len_hex);
        let list_len_hex_byte = hex_to_byte(list_len_hex);
        orderbook_bytes.extend_from_slice(&list_len_hex_byte);

        for order in order_list {
            //3）data(price(5)、quant(5)) 10*dataLen BYTE[10*dataLen] 信息体
            let price = order.price;
            let quantity_base = order.quantity_base;

            let price_bytes = encode_num_to_bytes(price.to_string());
            let quantity_base_bytes = encode_num_to_bytes(quantity_base.to_string());
            orderbook_bytes.extend_from_slice(&price_bytes);
            orderbook_bytes.extend_from_slice(&quantity_base_bytes);
        }
    }

    // let compressed = compress_to_vec(&bytes, 6);
    // println!("compressed from {} to {}", data.len(), compressed.len());
    orderbook_bytes
}

fn decode_orderbook(payload: Vec<i8>) -> OrderBookMsg {
    let mut data_byte_index = 0;

    //1、交易所时间戳:6 or 8 字节时间戳
    let mut exchange_timestamp_array: [i8; 16] = [0; 16];
    exchange_timestamp_array[10..].copy_from_slice(&payload[0..6]);

    let exchange_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(exchange_timestamp_array) };
    let exchange_timestamp = i128::from_be_bytes(exchange_timestamp_array);
    data_byte_index += 6;

    //2、收到时间戳:6 or 8 字节时间戳
    let mut received_timestamp_array: [i8; 16] = [0; 16];
    received_timestamp_array[10..].copy_from_slice(&payload[0..6]);
    let received_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(received_timestamp_array) };
    let received_timestamp = i128::from_be_bytes(received_timestamp_array);
    data_byte_index += 6;

    //3、EXANGE 1字节信息标识
    let exchange = payload.get(data_byte_index);
    data_byte_index += 1;
    let exchange_name = match exchange.unwrap() {
        1 => "crypto",
        2 => "ftx",
        3 => "binance",
        _ => "unknow",
    };

    //4、MARKET_TYPE 1字节信息标识
    let market_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let market_type_name = match market_type.unwrap() {
        1 => MarketType::Spot,
        2 => MarketType::LinearFuture,
        3 => MarketType::InverseFuture,
        4 => MarketType::LinearSwap,
        5 => MarketType::InverseSwap,
        6 => MarketType::EuropeanOption,
        7 => MarketType::AmericanOption,
        _ => MarketType::Unknown,
    };

    //5、MESSAGE_TYPE 1字节信息标识
    let message_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let message_type_name = match message_type.unwrap() {
        1 => MessageType::Trade,
        2 => MessageType::BBO,
        3 => MessageType::L2TopK,
        4 => MessageType::L2Snapshot,
        5 => MessageType::L2Event,
        6 => MessageType::L3Snapshot,
        7 => MessageType::L3Event,
        8 => MessageType::Ticker,
        9 => MessageType::Candlestick,
        10 => MessageType::OpenInterest,
        11 => MessageType::FundingRate,
        12 => MessageType::Other,
        _ => MessageType::Other,
    };

    //6、SYMBLE 2字节信息标识
    let symbol_bytes = &payload[data_byte_index..data_byte_index + 2];
    data_byte_index += 2;
    let mut symbol_bytes_dst = [0i8; 2];
    symbol_bytes_dst.clone_from_slice(symbol_bytes);
    let symbol_bytes_dst = unsafe { std::mem::transmute::<[i8; 2], [u8; 2]>(symbol_bytes_dst) };
    let symbol = i16::from_be_bytes(symbol_bytes_dst);
    let pair = match symbol {
        1 => "BTC/USDT",
        2 => "BTC/USD",
        3 => "USDT/USD",
        _ => "UNKNOWN",
    };

    //7、ask、bid
    let mut asks: Vec<Order> = Vec::new();
    let mut bids: Vec<Order> = Vec::new();
    while data_byte_index < payload.len() {
        //1）字节信息标识
        let data_type_flag = payload.get(data_byte_index);
        data_byte_index += 1;

        //2）字节信息体的长度
        let info_bytes_len = &payload[data_byte_index..data_byte_index + 2];
        data_byte_index += 2;
        let mut info_bytes_dst = [0i8; 2];
        info_bytes_dst.clone_from_slice(info_bytes_len);
        let info_bytes_dst = unsafe { std::mem::transmute::<[i8; 2], [u8; 2]>(info_bytes_dst) };
        let mut info_len = u16::from_be_bytes(info_bytes_dst);
        info_len /= 10;

        let mut i = 0;
        while i < info_len {
            // price
            let mut price_array: [i8; 8] = [0; 8];
            price_array[4..].copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
            let price_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(price_array) };
            let price_int = i64::from_be_bytes(price_array);

            let price_hex_p = payload[data_byte_index + 4];
            let price_hex_p_array = [price_hex_p];
            let mut price_p_array: [i8; 4] = [0; 4];
            price_p_array[3] = price_hex_p_array[0];
            let price_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(price_p_array) };
            let price_p_int = u32::from_be_bytes(price_p_array);

            let price = Decimal::new(price_int, price_p_int);
            let pricef = price.to_f64();

            // quant
            let mut quant_array = [0i8; 8];
            quant_array[4..]
                .copy_from_slice(&payload[data_byte_index + 5..data_byte_index + 5 + 4]);
            let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
            let quant_int = i64::from_be_bytes(quant_array);

            let quant_hex_p = payload[data_byte_index + 5 + 4];
            let quant_hex_p_array = [quant_hex_p];
            let mut quant_p_array = [0i8; 4];
            quant_p_array[3] = quant_hex_p_array[0];
            let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
            let quant_p_int = u32::from_be_bytes(quant_p_array);

            let quant = Decimal::new(quant_int, quant_p_int);
            let quantf = quant.to_f64();

            let order = Order {
                price: pricef.unwrap(),
                quantity_base: quantf.unwrap(),
                quantity_quote: 0.0,
                quantity_contract: None,
            };

            let data_type_flag_u8 = data_type_flag.unwrap().to_be();
            if 1 == data_type_flag_u8 {
                // ask
                asks.push(order);
            } else if (2 == data_type_flag_u8) {
                // bid
                bids.push(order);
            }

            i += 1;
            data_byte_index += 10
        }
    }

    let orderbook = OrderBookMsg {
        exchange: exchange_name.to_string(),
        market_type: market_type_name,
        symbol: pair.to_string(),
        pair: pair.to_string(),
        msg_type: message_type_name,
        timestamp: exchange_timestamp as i64,
        seq_id: None,
        prev_seq_id: None,
        asks: asks,
        bids: bids,
        snapshot: true,
        json: "".to_string(),
    };

    orderbook
}

pub fn encode_trade(orderbook: &TradeMsg) -> Vec<i8> {
    let mut orderbook_bytes: Vec<i8> = Vec::new();

    let exchange_timestamp = orderbook.timestamp;

    //1、交易所时间戳:6 or 8 字节时间戳
    let exchange_timestamp_hex = long_to_hex(exchange_timestamp);
    let exchange_timestamp_hex_byte = hex_to_byte(exchange_timestamp_hex);
    orderbook_bytes.extend_from_slice(&exchange_timestamp_hex_byte);

    //2、收到时间戳:6 or 8 字节时间戳
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let now_ms = now.as_millis();
    let received_timestamp_hex = long_to_hex(now_ms as i64);
    let received_timestamp_hex_byte = hex_to_byte(received_timestamp_hex);
    orderbook_bytes.extend_from_slice(&received_timestamp_hex_byte);

    //3、EXANGE 1字节信息标识
    let _exchange = (*EXANGE.get(&orderbook.exchange.as_str()).unwrap()) as i8;
    orderbook_bytes.push(_exchange);

    //4、MARKET_TYPE 1字节信息标识
    let _market_type = match orderbook.market_type {
        Spot => 1,
        LinearFuture => 2,
        InverseFuture => 3,
        LinearSwap => 4,
        InverseSwap => 5,
        EuropeanOption => 6,
    };
    orderbook_bytes.push(_market_type);

    //5、MESSAGE_TYPE 1字节信息标识
    let _message_type = match orderbook.msg_type {
        Trade => 1,
        BBO => 2,
        L2TopK => 3,
        L2Snapshot => 4,
        L2Event => 5,
        L3Snapshot => 6,
        L3Event => 7,
        Ticker => 8,
        Candlestick => 9,
        OpenInterest => 10,
        FundingRate => 11,
        Other => 12,
    };
    orderbook_bytes.push(_message_type);

    //6、SYMBLE 2字节信息标识
    let _pair = SYMBLE.get(&orderbook.pair.as_str()).unwrap();
    let _pair_hex = long_to_hex(*_pair as i64);
    let _pair_hex = format!("{:0>4}", _pair_hex);
    let _pair_hex_byte = hex_to_byte(_pair_hex);
    orderbook_bytes.extend_from_slice(&_pair_hex_byte);

    //7、TradeSide 1字节信息标识
    let _side = match orderbook.side {
        Buy => 1,
        Sell => 2,
    };
    orderbook_bytes.push(_side);

    //3）data(price(5)、quant(5))
    let price = orderbook.price;
    let quantity_base = orderbook.quantity_base;
    let price_bytes = encode_num_to_bytes(price.to_string());
    let quantity_base_bytes = encode_num_to_bytes(quantity_base.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);
    orderbook_bytes.extend_from_slice(&quantity_base_bytes);

    orderbook_bytes
}

fn decode_trade(payload: Vec<i8>) -> TradeMsg {

    let mut data_byte_index = 0;

    //1、交易所时间戳:6 or 8 字节时间戳
    let mut exchange_timestamp_array: [i8; 16] = [0; 16];
    exchange_timestamp_array[10..].copy_from_slice(&payload[0..6]);

    let exchange_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(exchange_timestamp_array) };
    let exchange_timestamp = i128::from_be_bytes(exchange_timestamp_array);
    data_byte_index += 6;

    //2、收到时间戳:6 or 8 字节时间戳
    let mut received_timestamp_array: [i8; 16] = [0; 16];
    received_timestamp_array[10..].copy_from_slice(&payload[0..6]);
    let received_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(received_timestamp_array) };
    let received_timestamp = i128::from_be_bytes(received_timestamp_array);
    data_byte_index += 6;

    //3、EXANGE 1字节信息标识
    let exchange = payload.get(data_byte_index);
    data_byte_index += 1;
    let exchange_name = match exchange.unwrap() {
        1 => "crypto",
        2 => "ftx",
        3 => "binance",
        11 => "okx",
        _ => "unknow",
    };

    //4、MARKET_TYPE 1字节信息标识
    let market_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let market_type_name = match market_type.unwrap() {
        1 => MarketType::Spot,
        2 => MarketType::LinearFuture,
        3 => MarketType::InverseFuture,
        4 => MarketType::LinearSwap,
        5 => MarketType::InverseSwap,
        6 => MarketType::EuropeanOption,
        7 => MarketType::AmericanOption,
        _ => MarketType::Unknown,
    };

    //5、MESSAGE_TYPE 1字节信息标识
    let message_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let message_type_name = match message_type.unwrap() {
        1 => MessageType::Trade,
        2 => MessageType::BBO,
        3 => MessageType::L2TopK,
        4 => MessageType::L2Snapshot,
        5 => MessageType::L2Event,
        6 => MessageType::L3Snapshot,
        7 => MessageType::L3Event,
        8 => MessageType::Ticker,
        9 => MessageType::Candlestick,
        10 => MessageType::OpenInterest,
        11 => MessageType::FundingRate,
        12 => MessageType::Other,
        _ => MessageType::Other,
    };

    //6、SYMBLE 2字节信息标识
    let symbol_bytes = &payload[data_byte_index..data_byte_index + 2];
    data_byte_index += 2;
    let mut symbol_bytes_dst = [0i8; 2];
    symbol_bytes_dst.clone_from_slice(symbol_bytes);
    let symbol_bytes_dst = unsafe { std::mem::transmute::<[i8; 2], [u8; 2]>(symbol_bytes_dst) };
    let symbol = i16::from_be_bytes(symbol_bytes_dst);
    let pair = match symbol {
        1 => "BTC/USDT",
        2 => "BTC/USD",
        3 => "USDT/USD",
        _ => "UNKNOWN",
    };

    //7、TradeSide 1字节信息标识
    let trade_side_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let trade_side = match trade_side_type.unwrap() {
        1 => TradeSide::Buy,
        2 => TradeSide::Sell,
        _ => TradeSide::Sell,
    };

    // price
    let mut price_array: [i8; 8] = [0; 8];
    price_array[4..].copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let price_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(price_array) };
    let price_int = i64::from_be_bytes(price_array);

    let price_hex_p = payload[data_byte_index + 4];
    let price_hex_p_array = [price_hex_p];
    let mut price_p_array: [i8; 4] = [0; 4];
    price_p_array[3] = price_hex_p_array[0];
    let price_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(price_p_array) };
    let price_p_int = u32::from_be_bytes(price_p_array);

    let price = Decimal::new(price_int, price_p_int);
    let pricef = price.to_f64();

    data_byte_index += 5;

    // quant
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let quantf = quant.to_f64();

    let trade = TradeMsg {
        exchange: exchange_name.to_string(),
        market_type: market_type_name,
        msg_type: message_type_name,
        pair: pair.to_string(),
        symbol: pair.to_string(),
        timestamp: exchange_timestamp as i64,
        side: trade_side,
        price: pricef.unwrap(),
        quantity_base: quantf.unwrap(),
        quantity_quote: 0.0,
        quantity_contract: None,
        trade_id: "".to_string(),
        json: "".to_string(),
    };

    trade
}


pub fn encode_bbo(orderbook: &BboMsg) -> Vec<i8> {
    let mut orderbook_bytes: Vec<i8> = Vec::new();

    let exchange_timestamp = orderbook.timestamp;

    //1、交易所时间戳:6 or 8 字节时间戳
    let exchange_timestamp_hex = long_to_hex(exchange_timestamp);
    let exchange_timestamp_hex_byte = hex_to_byte(exchange_timestamp_hex);
    orderbook_bytes.extend_from_slice(&exchange_timestamp_hex_byte);

    //2、收到时间戳:6 or 8 字节时间戳
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let now_ms = now.as_millis();
    let received_timestamp_hex = long_to_hex(now_ms as i64);
    let received_timestamp_hex_byte = hex_to_byte(received_timestamp_hex);
    orderbook_bytes.extend_from_slice(&received_timestamp_hex_byte);

    //3、EXANGE 1字节信息标识
    let _exchange = (*EXANGE.get(&orderbook.exchange.as_str()).unwrap()) as i8;
    orderbook_bytes.push(_exchange);

    //4、MARKET_TYPE 1字节信息标识
    let _market_type = match orderbook.market_type {
        Spot => 1,
        LinearFuture => 2,
        InverseFuture => 3,
        LinearSwap => 4,
        InverseSwap => 5,
        EuropeanOption => 6,
    };
    orderbook_bytes.push(_market_type);

    //5、MESSAGE_TYPE 1字节信息标识
    let _message_type = match orderbook.msg_type {
        Trade => 1,
        BBO => 2,
        L2TopK => 3,
        L2Snapshot => 4,
        L2Event => 5,
        L3Snapshot => 6,
        L3Event => 7,
        Ticker => 8,
        Candlestick => 9,
        OpenInterest => 10,
        FundingRate => 11,
        Other => 12,
    };
    orderbook_bytes.push(_message_type);

    //6、SYMBLE 2字节信息标识
    let _pair = SYMBLE.get(&orderbook.pair.as_str()).unwrap();
    let _pair_hex = long_to_hex(*_pair as i64);
    let _pair_hex = format!("{:0>4}", _pair_hex);
    let _pair_hex_byte = hex_to_byte(_pair_hex);
    orderbook_bytes.extend_from_slice(&_pair_hex_byte);

    //7、ask price(5)、quant(5)
    let price = orderbook.ask_price;
    let quantity_base = orderbook.ask_quantity_base;
    let price_bytes = encode_num_to_bytes(price.to_string());
    let quantity_base_bytes = encode_num_to_bytes(quantity_base.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);
    orderbook_bytes.extend_from_slice(&quantity_base_bytes);

    //8、bid price(5)、quant(5)
    let price = orderbook.bid_price;
    let quantity_base = orderbook.bid_quantity_base;
    let price_bytes = encode_num_to_bytes(price.to_string());
    let quantity_base_bytes = encode_num_to_bytes(quantity_base.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);
    orderbook_bytes.extend_from_slice(&quantity_base_bytes);

    orderbook_bytes
}

fn decode_bbo(payload: Vec<i8>) -> BboMsg {

    let mut data_byte_index = 0;

    //1、交易所时间戳:6 or 8 字节时间戳
    let mut exchange_timestamp_array: [i8; 16] = [0; 16];
    exchange_timestamp_array[10..].copy_from_slice(&payload[0..6]);

    let exchange_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(exchange_timestamp_array) };
    let exchange_timestamp = i128::from_be_bytes(exchange_timestamp_array);
    data_byte_index += 6;

    //2、收到时间戳:6 or 8 字节时间戳
    let mut received_timestamp_array: [i8; 16] = [0; 16];
    received_timestamp_array[10..].copy_from_slice(&payload[0..6]);
    let received_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(received_timestamp_array) };
    let received_timestamp = i128::from_be_bytes(received_timestamp_array);
    data_byte_index += 6;

    //3、EXANGE 1字节信息标识
    let exchange = payload.get(data_byte_index);
    data_byte_index += 1;
    let exchange_name = match exchange.unwrap() {
        1 => "crypto",
        2 => "ftx",
        3 => "binance",
        11 => "okx",
        _ => "unknow",
    };

    //4、MARKET_TYPE 1字节信息标识
    let market_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let market_type_name = match market_type.unwrap() {
        1 => MarketType::Spot,
        2 => MarketType::LinearFuture,
        3 => MarketType::InverseFuture,
        4 => MarketType::LinearSwap,
        5 => MarketType::InverseSwap,
        6 => MarketType::EuropeanOption,
        7 => MarketType::AmericanOption,
        _ => MarketType::Unknown,
    };

    //5、MESSAGE_TYPE 1字节信息标识
    let message_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let message_type_name = match message_type.unwrap() {
        1 => MessageType::Trade,
        2 => MessageType::BBO,
        3 => MessageType::L2TopK,
        4 => MessageType::L2Snapshot,
        5 => MessageType::L2Event,
        6 => MessageType::L3Snapshot,
        7 => MessageType::L3Event,
        8 => MessageType::Ticker,
        9 => MessageType::Candlestick,
        10 => MessageType::OpenInterest,
        11 => MessageType::FundingRate,
        12 => MessageType::Other,
        _ => MessageType::Other,
    };

    //6、SYMBLE 2字节信息标识
    let symbol_bytes = &payload[data_byte_index..data_byte_index + 2];
    data_byte_index += 2;
    let mut symbol_bytes_dst = [0i8; 2];
    symbol_bytes_dst.clone_from_slice(symbol_bytes);
    let symbol_bytes_dst = unsafe { std::mem::transmute::<[i8; 2], [u8; 2]>(symbol_bytes_dst) };
    let symbol = i16::from_be_bytes(symbol_bytes_dst);
    let pair = match symbol {
        1 => "BTC/USDT",
        2 => "BTC/USD",
        3 => "USDT/USD",
        _ => "UNKNOWN",
    };

    // ask price
    let mut price_array: [i8; 8] = [0; 8];
    price_array[4..].copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let price_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(price_array) };
    let price_int = i64::from_be_bytes(price_array);

    let price_hex_p = payload[data_byte_index + 4];
    let price_hex_p_array = [price_hex_p];
    let mut price_p_array: [i8; 4] = [0; 4];
    price_p_array[3] = price_hex_p_array[0];
    let price_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(price_p_array) };
    let price_p_int = u32::from_be_bytes(price_p_array);

    let price = Decimal::new(price_int, price_p_int);
    let ask_pricef = price.to_f64();

    data_byte_index += 5;

    // ask quant
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let ask_quantf = quant.to_f64();

    data_byte_index += 5;

    // bid price
    let mut price_array: [i8; 8] = [0; 8];
    price_array[4..].copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let price_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(price_array) };
    let price_int = i64::from_be_bytes(price_array);

    let price_hex_p = payload[data_byte_index + 4];
    let price_hex_p_array = [price_hex_p];
    let mut price_p_array: [i8; 4] = [0; 4];
    price_p_array[3] = price_hex_p_array[0];
    let price_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(price_p_array) };
    let price_p_int = u32::from_be_bytes(price_p_array);

    let price = Decimal::new(price_int, price_p_int);
    let bid_pricef = price.to_f64();

    data_byte_index += 5;

    // bid quant
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let bid_quantf = quant.to_f64();

    let bbo = BboMsg {
        exchange: exchange_name.to_string(),
        market_type: market_type_name,
        msg_type: message_type_name,
        pair: pair.to_string(),
        symbol: pair.to_string(),
        timestamp: exchange_timestamp as i64,
        ask_price: ask_pricef.unwrap(),
        ask_quantity_base: ask_quantf.unwrap(),
        ask_quantity_quote: 0.0,
        ask_quantity_contract: None,
        bid_price: bid_pricef.unwrap(),
        bid_quantity_base: bid_quantf.unwrap(),
        bid_quantity_quote: 0.0,
        bid_quantity_contract: None,
        id: None,
        json: "".to_string(),
    };

    bbo
}


pub fn encode_kline(orderbook: &KlineMsg) -> Vec<i8> {
    let mut orderbook_bytes: Vec<i8> = Vec::new();

    let exchange_timestamp = orderbook.timestamp;

    //1、交易所时间戳:6 or 8 字节时间戳
    let exchange_timestamp_hex = long_to_hex(exchange_timestamp);
    let exchange_timestamp_hex_byte = hex_to_byte(exchange_timestamp_hex);
    orderbook_bytes.extend_from_slice(&exchange_timestamp_hex_byte);

    //2、收到时间戳:6 or 8 字节时间戳
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let now_ms = now.as_millis();
    let received_timestamp_hex = long_to_hex(now_ms as i64);
    let received_timestamp_hex_byte = hex_to_byte(received_timestamp_hex);
    orderbook_bytes.extend_from_slice(&received_timestamp_hex_byte);

    //3、EXANGE 1字节信息标识
    let _exchange = (*EXANGE.get(&orderbook.exchange.as_str()).unwrap()) as i8;
    orderbook_bytes.push(_exchange);

    //4、MARKET_TYPE 1字节信息标识
    let _market_type = match orderbook.market_type {
        Spot => 1,
        LinearFuture => 2,
        InverseFuture => 3,
        LinearSwap => 4,
        InverseSwap => 5,
        EuropeanOption => 6,
    };
    orderbook_bytes.push(_market_type);

    //5、MESSAGE_TYPE 1字节信息标识
    let _message_type = match orderbook.msg_type {
        Trade => 1,
        BBO => 2,
        L2TopK => 3,
        L2Snapshot => 4,
        L2Event => 5,
        L3Snapshot => 6,
        L3Event => 7,
        Ticker => 8,
        Candlestick => 9,
        OpenInterest => 10,
        FundingRate => 11,
        Other => 12,
    };
    orderbook_bytes.push(_message_type);

    //6、SYMBLE 2字节信息标识
    let _pair = SYMBLE.get(&orderbook.pair.as_str()).unwrap();
    let _pair_hex = long_to_hex(*_pair as i64);
    let _pair_hex = format!("{:0>4}", _pair_hex);
    let _pair_hex_byte = hex_to_byte(_pair_hex);
    orderbook_bytes.extend_from_slice(&_pair_hex_byte);

    //7、PERIOD 1字节信息标识
    let _period = (*PERIOD.get(&orderbook.period.as_str()).unwrap()) as i8;
    orderbook_bytes.push(_period);
    
    //8、open
    let price = orderbook.open;
    let price_bytes = encode_num_to_bytes(price.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);

    //9、high
    let price = orderbook.high;
    let price_bytes = encode_num_to_bytes(price.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);

    //10、low
    let price = orderbook.low;
    let price_bytes = encode_num_to_bytes(price.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);

    //11、close
    let price = orderbook.close;
    let price_bytes = encode_num_to_bytes(price.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);

    //12、volume
    let price = orderbook.volume;
    let price_bytes = encode_num_to_bytes(price.to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);

    //13、volume
    let price = orderbook.quote_volume;
    let price_bytes = encode_num_to_bytes(price.unwrap().to_string());
    orderbook_bytes.extend_from_slice(&price_bytes);


    orderbook_bytes
}

fn decode_kline(payload: Vec<i8>) -> KlineMsg {

    let mut data_byte_index = 0;

    //1、交易所时间戳:6 or 8 字节时间戳
    let mut exchange_timestamp_array: [i8; 16] = [0; 16];
    exchange_timestamp_array[10..].copy_from_slice(&payload[0..6]);

    let exchange_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(exchange_timestamp_array) };
    let exchange_timestamp = i128::from_be_bytes(exchange_timestamp_array);
    data_byte_index += 6;

    //2、收到时间戳:6 or 8 字节时间戳
    let mut received_timestamp_array: [i8; 16] = [0; 16];
    received_timestamp_array[10..].copy_from_slice(&payload[0..6]);
    let received_timestamp_array =
        unsafe { std::mem::transmute::<[i8; 16], [u8; 16]>(received_timestamp_array) };
    let received_timestamp = i128::from_be_bytes(received_timestamp_array);
    data_byte_index += 6;

    //3、EXANGE 1字节信息标识
    let exchange = payload.get(data_byte_index);
    data_byte_index += 1;
    let exchange_name = match exchange.unwrap() {
        1 => "crypto",
        2 => "ftx",
        3 => "binance",
        11 => "okx",
        _ => "unknow",
    };

    //4、MARKET_TYPE 1字节信息标识
    let market_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let market_type_name = match market_type.unwrap() {
        1 => MarketType::Spot,
        2 => MarketType::LinearFuture,
        3 => MarketType::InverseFuture,
        4 => MarketType::LinearSwap,
        5 => MarketType::InverseSwap,
        6 => MarketType::EuropeanOption,
        7 => MarketType::AmericanOption,
        _ => MarketType::Unknown,
    };

    //5、MESSAGE_TYPE 1字节信息标识
    let message_type = payload.get(data_byte_index);
    data_byte_index += 1;
    let message_type_name = match message_type.unwrap() {
        1 => MessageType::Trade,
        2 => MessageType::BBO,
        3 => MessageType::L2TopK,
        4 => MessageType::L2Snapshot,
        5 => MessageType::L2Event,
        6 => MessageType::L3Snapshot,
        7 => MessageType::L3Event,
        8 => MessageType::Ticker,
        9 => MessageType::Candlestick,
        10 => MessageType::OpenInterest,
        11 => MessageType::FundingRate,
        12 => MessageType::Other,
        _ => MessageType::Other,
    };

    //6、SYMBLE 2字节信息标识
    let symbol_bytes = &payload[data_byte_index..data_byte_index + 2];
    data_byte_index += 2;
    let mut symbol_bytes_dst = [0i8; 2];
    symbol_bytes_dst.clone_from_slice(symbol_bytes);
    let symbol_bytes_dst = unsafe { std::mem::transmute::<[i8; 2], [u8; 2]>(symbol_bytes_dst) };
    let symbol = i16::from_be_bytes(symbol_bytes_dst);
    let pair = match symbol {
        1 => "BTC/USDT",
        2 => "BTC/USD",
        3 => "USDT/USD",
        _ => "UNKNOWN",
    };

    //7、PERIOD 1字节信息标识
    let period = payload.get(data_byte_index);
    data_byte_index += 1;
    let period_name = match period.unwrap() {
        1 => "1m",
        2 => "5m",
        3 => "30m",
        4 => "1h",
        _ => "unknow",
    };

    // open
    let mut price_array: [i8; 8] = [0; 8];
    price_array[4..].copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let price_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(price_array) };
    let price_int = i64::from_be_bytes(price_array);

    let price_hex_p = payload[data_byte_index + 4];
    let price_hex_p_array = [price_hex_p];
    let mut price_p_array: [i8; 4] = [0; 4];
    price_p_array[3] = price_hex_p_array[0];
    let price_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(price_p_array) };
    let price_p_int = u32::from_be_bytes(price_p_array);

    let price = Decimal::new(price_int, price_p_int);
    let open_pricef = price.to_f64();

    data_byte_index += 5;

    // high
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let high_pricef = quant.to_f64();

    data_byte_index += 5;

    // low price
    let mut price_array: [i8; 8] = [0; 8];
    price_array[4..].copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let price_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(price_array) };
    let price_int = i64::from_be_bytes(price_array);

    let price_hex_p = payload[data_byte_index + 4];
    let price_hex_p_array = [price_hex_p];
    let mut price_p_array: [i8; 4] = [0; 4];
    price_p_array[3] = price_hex_p_array[0];
    let price_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(price_p_array) };
    let price_p_int = u32::from_be_bytes(price_p_array);

    let price = Decimal::new(price_int, price_p_int);
    let low_pricef = price.to_f64();

    data_byte_index += 5;

    // close
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let close_pricef = quant.to_f64();

    data_byte_index += 5;

    // volume
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let volume_pricef = quant.to_f64();

    // quote_volume
    let mut quant_array = [0i8; 8];
    quant_array[4..]
        .copy_from_slice(&payload[data_byte_index..data_byte_index + 4]);
    let quant_array = unsafe { std::mem::transmute::<[i8; 8], [u8; 8]>(quant_array) };
    let quant_int = i64::from_be_bytes(quant_array);

    let quant_hex_p = payload[data_byte_index + 4];
    let quant_hex_p_array = [quant_hex_p];
    let mut quant_p_array = [0i8; 4];
    quant_p_array[3] = quant_hex_p_array[0];
    let quant_p_array = unsafe { std::mem::transmute::<[i8; 4], [u8; 4]>(quant_p_array) };
    let quant_p_int = u32::from_be_bytes(quant_p_array);

    let quant = Decimal::new(quant_int, quant_p_int);
    let quote_volume_pricef = quant.to_f64();

    let kline = KlineMsg {
        exchange: exchange_name.to_string(),
        market_type: market_type_name,
        msg_type: message_type_name,
        pair: pair.to_string(),
        symbol: pair.to_string(),
        timestamp: exchange_timestamp as i64,
        open: open_pricef.unwrap(),
        high: high_pricef.unwrap(),
        low: low_pricef.unwrap(),
        close: close_pricef.unwrap(),
        /// base volume
        volume: volume_pricef.unwrap(),
        /// m, minute; H, hour; D, day; W, week; M, month; Y, year
        period: period_name.to_string(),
        /// quote volume
        quote_volume: quote_volume_pricef,
        json: "".to_string(),
    };

    kline
}