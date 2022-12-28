use std::collections::HashSet;

use crypto_market_type::MarketType;

pub fn get_hot_spot_symbols(exchange: &str, spot_symbols: &[String]) -> Vec<String> {
    let market_types = crypto_market_type::get_market_types(exchange);
    let cmc_ranks = &super::cmc_rank::CMC_RANKS;
    let contract_base_coins = {
        let mut contract_base_coins = HashSet::<String>::new();
        for market_type in market_types.iter().filter(|m| *m != &MarketType::Spot) {
            let symbols = crypto_markets::fetch_symbols(exchange, *market_type).unwrap_or_default();
            for symbol in symbols {
                let pair = crypto_pair::normalize_pair(&symbol, exchange).unwrap();
                let base_coin = pair.split('/').next().unwrap();
                contract_base_coins.insert(base_coin.to_string());
            }
        }
        contract_base_coins
    };
    let is_hot = |symbol: &str| {
        let pair = crypto_pair::normalize_pair(symbol, exchange).unwrap();
        let base_coin = pair.split('/').next().unwrap();
        contract_base_coins.contains(base_coin)
            || *cmc_ranks.get(base_coin).unwrap_or(&u64::max_value()) <= 100
    };

    spot_symbols.iter().cloned().filter(|symbol| is_hot(symbol)).collect()
}

#[cfg(test)]
mod tests {
    use crypto_market_type::MarketType;

    use super::get_hot_spot_symbols;

    #[test]
    fn test_binance() {
        let spot_symbols = crypto_markets::fetch_symbols("binance", MarketType::Spot).unwrap();
        let symbols = get_hot_spot_symbols("binance", &spot_symbols);
        assert!(!symbols.is_empty());
    }

    #[test]
    fn test_huobi() {
        let spot_symbols = crypto_markets::fetch_symbols("huobi", MarketType::Spot).unwrap();
        let symbols = get_hot_spot_symbols("huobi", &spot_symbols);
        assert!(!symbols.is_empty());
    }

    #[test]
    fn test_okx() {
        let spot_symbols = crypto_markets::fetch_symbols("okx", MarketType::Spot).unwrap();
        let symbols = get_hot_spot_symbols("okx", &spot_symbols);
        assert!(!symbols.is_empty());
    }
}
