use crypto_msg_parser::{parse_l2, parse_trade, MarketType, MessageType};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::Value;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::{collections::HashMap, env};

fn parse_lines(buf_reader: &mut dyn std::io::BufRead, writer: &mut dyn std::io::Write) -> i64 {
    let mut error_lines = 0;
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            if let Ok(json_obj) = serde_json::from_str::<HashMap<String, Value>>(&line) {
                let exchange = json_obj
                    .get("exchange")
                    .expect("No exchange field!")
                    .as_str()
                    .unwrap();
                let market_type = json_obj
                    .get("market_type")
                    .expect("No market_type field!")
                    .as_str()
                    .unwrap();
                let msg_type = json_obj
                    .get("msg_type")
                    .expect("No msg_type field!")
                    .as_str()
                    .unwrap();
                let timestamp = json_obj
                    .get("received_at")
                    .expect("No received_at field!")
                    .as_i64();
                let raw = json_obj
                    .get("json")
                    .expect("No json field!")
                    .as_str()
                    .unwrap();

                let market_type = MarketType::from_str(market_type).unwrap();
                let msg_type = MessageType::from_str(msg_type).unwrap();

                match msg_type {
                    MessageType::L2Event => {
                        if let Ok(messages) = parse_l2(exchange, market_type, raw, timestamp) {
                            for message in messages {
                                let json_str = serde_json::to_string(&message).unwrap();
                                writeln!(writer, "{}\n", json_str).unwrap();
                            }
                        }
                    }
                    MessageType::Trade => {
                        if let Ok(messages) = parse_trade(exchange, market_type, raw) {
                            for message in messages {
                                let json_str = serde_json::to_string(&message).unwrap();
                                writeln!(writer, "{}", json_str).unwrap();
                            }
                        }
                    }
                    _ => panic!("Unknown msg_type {}", msg_type),
                }
            } else {
                error_lines += 1;
            }
        } else {
            error_lines += 1;
        }
    }
    writer.flush().unwrap();
    error_lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: crypto-msg-parser <input_file> <output_file>");
        return;
    }

    let input_file: &'static str = Box::leak(args[1].clone().into_boxed_str());
    let output_file: &'static str = Box::leak(args[2].clone().into_boxed_str());
    if !Path::new(input_file).is_file() {
        println!("File {} does not exist", input_file);
        return;
    }

    let f_in = std::fs::File::open(input_file).unwrap();
    let mut buf_reader: Box<dyn std::io::BufRead> = if input_file.ends_with(".gz") {
        let d = GzDecoder::new(f_in);
        Box::new(std::io::BufReader::new(d))
    } else {
        Box::new(std::io::BufReader::new(f_in))
    };

    let output_dir = std::path::Path::new(output_file).parent().unwrap();
    std::fs::create_dir_all(output_dir).unwrap();
    let f_out = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)
        .unwrap();
    let mut writer: Box<dyn std::io::Write> = if output_file.ends_with(".gz") {
        let encoder = GzEncoder::new(f_out, Compression::best());
        Box::new(std::io::BufWriter::new(encoder))
    } else {
        Box::new(std::io::BufWriter::new(f_out))
    };

    let error_lines = parse_lines(buf_reader.as_mut(), writer.as_mut());
    println!("Parse succeeded, but dropped {} malformed lines", error_lines);
}
