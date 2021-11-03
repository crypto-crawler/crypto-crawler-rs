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
        for market in markets {
            let contract_value = crypto_contract_value::get_contract_value(
                &market.exchange,
                $market_type,
                format!("{}/{}", market.base, market.quote).as_str(),
            );
            assert_eq!(market.contract_value, contract_value);
        }
    }};
}
