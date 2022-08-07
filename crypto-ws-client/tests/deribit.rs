use crypto_ws_client::{DeribitWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn deribit_all_trades() {
    gen_test_code!(
        DeribitWSClient,
        subscribe,
        // https://docs.deribit.com/?javascript#trades-kind-currency-interval
        &[
            ("trades.future.SYMBOL.100ms".to_string(), "any".to_string()),
            ("trades.option.SYMBOL.100ms".to_string(), "any".to_string())
        ]
    );
}

#[cfg(test)]
mod deribit_inverse_future {
    use crypto_ws_client::{DeribitWSClient, WSClient};

    #[ignore = "lack of liquidity"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &[("trades.future.SYMBOL.100ms".to_string(), "BTC".to_string())]
        );
    }

    #[ignore = "lack of liquidity"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &["BTC-26AUG22".to_string(), "BTC-30SEP22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &["BTC-30SEP22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &["BTC-30SEP22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_topk,
            &["BTC-30SEP22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(DeribitWSClient, subscribe_bbo, &["BTC-30SEP22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(DeribitWSClient, &[("BTC-30SEP22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(DeribitWSClient, &[("BTC-30SEP22".to_string(), 86400)]);
    }
}

#[cfg(test)]
mod deribit_inverse_swap {
    use crypto_ws_client::{DeribitWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &[(
                "trades.SYMBOL.100ms".to_string(),
                "BTC-PERPETUAL".to_string()
            )]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &["BTC-PERPETUAL".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &["BTC-PERPETUAL".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &["BTC-PERPETUAL".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_topk,
            &["BTC-PERPETUAL".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_bbo,
            &["BTC-PERPETUAL".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(DeribitWSClient, &[("BTC-PERPETUAL".to_string(), 60)]);
        gen_test_subscribe_candlestick!(DeribitWSClient, &[("BTC-PERPETUAL".to_string(), 86400)]);
    }
}

#[cfg(test)]
mod deribit_option {
    use crypto_ws_client::{DeribitWSClient, WSClient};

    const SYMBOLS: &[&str] = &[
        "BTC-26AUG22-23000-C",
        "BTC-26AUG22-45000-C",
        "BTC-30SEP22-40000-C",
        "BTC-30SEP22-60000-C",
    ];

    #[ignore = "lack of liquidity"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &[("trades.option.SYMBOL.100ms".to_string(), "any".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_topk,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_bbo,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            SYMBOLS
                .iter()
                .map(|s| (s.to_string(), 60))
                .collect::<Vec<(String, usize)>>()
                .as_slice()
        );
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            SYMBOLS
                .iter()
                .map(|s| (s.to_string(), 86400))
                .collect::<Vec<(String, usize)>>()
                .as_slice()
        );
    }
}
