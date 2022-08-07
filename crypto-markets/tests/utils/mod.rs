#[allow(unused_macros)]
macro_rules! gen_all_symbols {
    () => {
        let market_types = get_market_types(EXCHANGE_NAME);
        assert!(!market_types.is_empty());

        for market_type in market_types
            .into_iter()
            .filter(|m| m != &MarketType::Unknown)
        {
            let symbols = fetch_symbols(EXCHANGE_NAME, market_type).unwrap();
            assert!(!symbols.is_empty());
        }
    };
}

#[allow(unused_macros)]
macro_rules! check_contract_values {
    ($exchange:expr, $market_type:expr) => {{
        let markets = fetch_markets($exchange, $market_type).unwrap();
        for market in markets.into_iter().filter(|m| {
            m.market_type == MarketType::InverseSwap
                || m.market_type == MarketType::LinearSwap
                || m.market_type == MarketType::InverseFuture
                || m.market_type == MarketType::LinearFuture
        }) {
            let contract_value = crypto_contract_value::get_contract_value(
                &market.exchange,
                market.market_type,
                format!("{}/{}", market.base, market.quote).as_str(),
            );
            assert_eq!(market.contract_value, contract_value);
            if market.base != crypto_pair::normalize_currency(market.base_id.as_str(), $exchange) {
                println!("{}", serde_json::to_string(&market).unwrap());
            }
            assert_eq!(
                market.base,
                crypto_pair::normalize_currency(market.base_id.as_str(), $exchange)
            );
            assert_eq!(
                market.quote,
                crypto_pair::normalize_currency(market.quote_id.as_str(), $exchange)
            );
            assert_eq!(
                market.settle.unwrap(),
                crypto_pair::normalize_currency(market.settle_id.unwrap().as_str(), $exchange)
            );
            // assert!(market.margin);
            if market.market_type == MarketType::InverseFuture
                || market.market_type == MarketType::LinearFuture
                || market.market_type == MarketType::QuantoFuture
                || market.market_type == MarketType::EuropeanOption
            {
                assert!(market.delivery_date.is_some());
            } else {
                assert!(market.delivery_date.is_none());
            }
        }
    }};
}
