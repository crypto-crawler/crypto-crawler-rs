mod exchanges;

pub use crypto_market_type::MarketType;

pub fn get_contract_value(exchange: &str, market_type: MarketType, pair: &str) -> Option<f64> {
    if market_type == MarketType::Spot {
        return Some(1.0);
    }

    match exchange {
        "binance" => exchanges::binance::get_contract_value(market_type, pair),
        "bitfinex" => exchanges::bitfinex::get_contract_value(market_type, pair),
        "bitget" => exchanges::bitget::get_contract_value(market_type, pair),
        "bitmex" => exchanges::bitmex::get_contract_value(market_type, pair),
        "bybit" => exchanges::bybit::get_contract_value(market_type, pair),
        "deribit" => exchanges::deribit::get_contract_value(market_type, pair),
        "ftx" => exchanges::ftx::get_contract_value(market_type, pair),
        "gate" => exchanges::gate::get_contract_value(market_type, pair),
        "huobi" => exchanges::huobi::get_contract_value(market_type, pair),
        "kucoin" => exchanges::kucoin::get_contract_value(market_type, pair),
        "mxc" => exchanges::mxc::get_contract_value(market_type, pair),
        "okex" => exchanges::okex::get_contract_value(market_type, pair),
        _ => panic!("Unknown exchange {}", exchange),
    }
}
