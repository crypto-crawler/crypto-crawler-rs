use crypto_msg_parser::{parse_l2, parse_trade, MarketType, MessageType, OrderBookMsg, TradeMsg};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::Value;
use std::io::prelude::*;
use std::str::FromStr;
use std::{collections::HashMap, env};

fn parse_lines(
    buf_reader: &mut dyn std::io::BufRead,
    writer: &mut dyn std::io::Write,
) -> (i64, i64) {
    let mut total_lines = 0;
    let mut error_lines = 0;
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            total_lines += 1;
            if let Ok(json_obj) = serde_json::from_str::<HashMap<String, Value>>(&line) {
                if !json_obj.contains_key("exchange")
                    && !json_obj.contains_key("market_type")
                    && !json_obj.contains_key("msg_type")
                {
                    error_lines += 1;
                } else {
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
                    let market_type = MarketType::from_str(market_type).unwrap();
                    let msg_type = MessageType::from_str(msg_type).unwrap();
                    if json_obj.contains_key("received_at") {
                        // raw messages from crypto-crawler
                        let timestamp = json_obj
                            .get("received_at")
                            .expect("No received_at field!")
                            .as_i64();
                        let raw = json_obj
                            .get("json")
                            .expect("No json field!")
                            .as_str()
                            .unwrap();
                        match msg_type {
                            MessageType::L2Event => {
                                if let Ok(messages) =
                                    parse_l2(exchange, market_type, raw, timestamp)
                                {
                                    for message in messages {
                                        let json_str = serde_json::to_string(&message).unwrap();
                                        writeln!(writer, "{}", json_str).unwrap();
                                    }
                                } else {
                                    error_lines += 1;
                                }
                            }
                            MessageType::Trade => {
                                if let Ok(messages) = parse_trade(exchange, market_type, raw) {
                                    for message in messages {
                                        let json_str = serde_json::to_string(&message).unwrap();
                                        writeln!(writer, "{}", json_str).unwrap();
                                    }
                                } else {
                                    error_lines += 1;
                                }
                            }
                            _ => panic!("Unknown msg_type {}", msg_type),
                        }
                    } else {
                        // re-parse OrderBookMsg and TradeMsg
                        match msg_type {
                            MessageType::L2Event => {
                                let msg = serde_json::from_str::<OrderBookMsg>(&line).unwrap();
                                let raw = serde_json::to_string(&msg.raw).unwrap();
                                if let Ok(messages) =
                                    parse_l2(exchange, market_type, &raw, Some(msg.timestamp))
                                {
                                    for message in messages {
                                        let json_str = serde_json::to_string(&message).unwrap();
                                        writeln!(writer, "{}", json_str).unwrap();
                                    }
                                } else {
                                    error_lines += 1;
                                }
                            }
                            MessageType::Trade => {
                                let msg = serde_json::from_str::<TradeMsg>(&line).unwrap();
                                let raw = serde_json::to_string(&msg.raw).unwrap();
                                if let Ok(messages) = parse_trade(exchange, market_type, &raw) {
                                    for message in messages {
                                        let json_str = serde_json::to_string(&message).unwrap();
                                        writeln!(writer, "{}", json_str).unwrap();
                                    }
                                } else {
                                    error_lines += 1;
                                }
                            }
                            _ => panic!("Unknown msg_type {}", msg_type),
                        }
                    }
                }
            } else {
                error_lines += 1;
            }
        } else {
            error_lines += 1;
        }
    }
    writer.flush().unwrap();
    (error_lines, total_lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: crypto-msg-parser <input_file> <output_file>");
        std::process::exit(1);
    }

    let input_file: &'static str = Box::leak(args[1].clone().into_boxed_str());
    let output_file: &'static str = Box::leak(args[2].clone().into_boxed_str());
    if !input_file.ends_with(".json")
        && !input_file.ends_with(".json.gz")
        && !input_file.ends_with(".json.xz")
    {
        eprintln!(
            "{} suffix should be .json, .json.gz or .json.xz",
            input_file
        );
        std::process::exit(1);
    }
    if !output_file.ends_with(".json")
        && !output_file.ends_with(".json.gz")
        && !output_file.ends_with(".json.xz")
    {
        eprintln!(
            "{} suffix should be .json, .json.gz or .json.xz",
            output_file
        );
        std::process::exit(1);
    }

    let f_in =
        std::fs::File::open(input_file).unwrap_or_else(|_| panic!("{} does not exist", input_file));
    let mut buf_reader: Box<dyn std::io::BufRead> = if input_file.ends_with(".json.gz") {
        let d = GzDecoder::new(f_in);
        Box::new(std::io::BufReader::new(d))
    } else if input_file.ends_with(".json.xz") {
        let d = xz2::read::XzDecoder::new(f_in);
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
    let mut writer: Box<dyn std::io::Write> = if output_file.ends_with(".json.gz") {
        let encoder = GzEncoder::new(f_out, Compression::best());
        Box::new(std::io::BufWriter::new(encoder))
    } else if output_file.ends_with(".json.xz") {
        let e = xz2::write::XzEncoder::new(f_out, 9);
        Box::new(std::io::BufWriter::new(e))
    } else {
        Box::new(std::io::BufWriter::new(f_out))
    };

    let (error_lines, total_lines) = parse_lines(buf_reader.as_mut(), writer.as_mut());
    println!(
        "Parse succeeded, dropped {} malformed lines out of {} lines",
        error_lines, total_lines
    );
}
