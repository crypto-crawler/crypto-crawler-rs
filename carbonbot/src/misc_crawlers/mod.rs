use std::sync::mpsc::Sender;

use crypto_crawler::Message;
use crypto_market_type::MarketType;

mod binance;
mod bitmex;
mod bybit;
mod coinbase_pro;
mod huobi;

mod utils;

pub async fn crawl_other(exchange: &str, market_type: MarketType, tx: Sender<Message>) {
    match exchange {
        "binance" => binance::crawl_other(market_type, tx).await,
        "bitmex" => bitmex::crawl_other(market_type, tx).await,
        "bybit" => bybit::crawl_other(market_type, tx).await,
        "coinbase_pro" => coinbase_pro::crawl_other(market_type, tx).await,
        "huobi" => huobi::crawl_other(market_type, tx).await,
        _ => panic!("Unknown exchange {}", exchange),
    }
}
