use crypto_market_type::MarketType;
use crypto_markets::fetch_markets;
use std::{env, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: crypto-markets <exchange> <market_type>");
        return;
    }

    let exchange: &str = &args[1];
    let market_type = MarketType::from_str(&args[2]);
    if market_type.is_err() {
        println!("Unknown market type: {}", &args[2]);
        return;
    }

    let resp = fetch_markets(exchange, market_type.unwrap());
    match resp {
        Ok(markets) => println!("{}", serde_json::to_string_pretty(&markets).unwrap()),
        Err(err) => println!("{}", err),
    }
}
