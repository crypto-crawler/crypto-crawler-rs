macro_rules! gen_all_symbols {
    () => {
        let market_types = get_market_types(EXCHANGE_NAME);
        assert!(!market_types.is_empty());

        for market_type in market_types.into_iter() {
            let symbols = fetch_symbols(EXCHANGE_NAME, market_type).unwrap();
            assert!(!symbols.is_empty());
        }
    };
}
