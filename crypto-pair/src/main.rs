use std::env;

use crypto_pair::normalize_pair;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: crypto-pair <symbol> [exchange]");
        return;
    }

    let exchanges = [
        "Biki",
        "Binance",
        "Bitfinex",
        "BitMEX",
        "Bitstamp",
        "CoinbasePro",
        "Coincheck",
        "Huobi",
        "Kraken",
        "MXC",
        "Newdex",
        "OKEx",
        "Poloniex",
        "Upbit",
        "WhaleEx",
        "Zaif",
        "ZB",
        "bitFlyer",
    ];
    let symbol: &str = &args[1];

    if args.len() == 3 {
        let exchange: &str = &args[2];
        let tmp: &str = &exchange;
        if !exchanges.contains(&tmp) {
            println!("{} is not in [{}]", exchange, exchanges.join(","));
        } else {
            println!("{}", normalize_pair(symbol, exchange).unwrap());
        }
    } else {
        println!("{}", normalize_pair(symbol, "").unwrap());
    }
}
