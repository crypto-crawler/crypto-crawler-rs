use async_trait::async_trait;

// tick-by-tick trade
#[async_trait]
pub(super) trait Trade {
    async fn subscribe_trade(&self, symbols: &[String]);
}

// 24hr rolling window ticker
#[async_trait]
pub(super) trait Ticker {
    async fn subscribe_ticker(&self, symbols: &[String]);
}

// Best Bid & Offer
#[allow(clippy::upper_case_acronyms)]
#[async_trait]
pub(super) trait BBO {
    async fn subscribe_bbo(&self, symbols: &[String]);
}

// An orderbook snapshot followed by realtime updates.
#[async_trait]
pub(super) trait OrderBook {
    async fn subscribe_orderbook(&self, symbols: &[String]);
}

#[async_trait]
pub(super) trait OrderBookTopK {
    /// Subscribes to level2 orderbook top-k snapshot channels.
    async fn subscribe_orderbook_topk(&self, symbols: &[String]);
}

/// Level3 orderbook data.
#[async_trait]
pub(super) trait Level3OrderBook {
    /// Subscribes to level3 orderebook channels.
    ///
    /// The level3 orderbook is the orginal orderbook of an exchange, it is
    /// non-aggregated by price level and updated tick-by-tick.
    async fn subscribe_l3_orderbook(&self, symbols: &[String]);
}

#[async_trait]
pub(super) trait Candlestick {
    /// Subscribes to candlestick channels which send OHLCV messages.
    ///
    /// `symbol_interval_list` is a list of symbols and intervals of candlesticks in seconds.
    async fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]);
}

macro_rules! impl_trait {
    ($trait_name:ident, $struct_name:ident, $method_name:ident, $channel:expr) => {
        #[async_trait]
        impl $trait_name for $struct_name {
            async fn $method_name(&self, symbols: &[String]) {
                let topics = symbols
                    .iter()
                    .map(|symbol| ($channel.to_string(), symbol.to_string()))
                    .collect::<Vec<(String, String)>>();
                self.subscribe(&topics).await;
            }
        }
    };
}

macro_rules! impl_candlestick {
    ($struct_name:ident) => {
        #[async_trait]
        impl Candlestick for $struct_name {
            async fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
                let commands = self
                    .translator
                    .translate_to_candlestick_commands(true, symbol_interval_list);
                self.client.send(&commands).await;
            }
        }
    };
}

macro_rules! panic_ticker {
    ($struct_name:ident) => {
        #[async_trait]
        impl Ticker for $struct_name {
            async fn subscribe_ticker(&self, _symbols: &[String]) {
                panic!(
                    "{} does NOT have the ticker websocket channel",
                    EXCHANGE_NAME
                );
            }
        }
    };
}

macro_rules! panic_bbo {
    ($struct_name:ident) => {
        #[async_trait]
        impl BBO for $struct_name {
            async fn subscribe_bbo(&self, _symbols: &[String]) {
                panic!("{} does NOT have the BBO websocket channel", EXCHANGE_NAME);
            }
        }
    };
}

macro_rules! panic_l2_topk {
    ($struct_name:ident) => {
        #[async_trait]
        impl OrderBookTopK for $struct_name {
            async fn subscribe_orderbook_topk(&self, _symbols: &[String]) {
                panic!(
                    "{} does NOT have the level2 top-k snapshot websocket channel",
                    EXCHANGE_NAME
                );
            }
        }
    };
}

macro_rules! panic_l3_orderbook {
    ($struct_name:ident) => {
        #[async_trait]
        impl Level3OrderBook for $struct_name {
            async fn subscribe_l3_orderbook(&self, _symbols: &[String]) {
                panic!(
                    "{} does NOT have the level3 websocket channel",
                    EXCHANGE_NAME
                );
            }
        }
    };
}

macro_rules! panic_candlestick {
    ($struct_name:ident) => {
        #[async_trait]
        impl Candlestick for $struct_name {
            async fn subscribe_candlestick(&self, _symbol_interval_list: &[(String, usize)]) {
                panic!(
                    "{} does NOT have the candlestick websocket channel",
                    EXCHANGE_NAME
                );
            }
        }
    };
}

/// Implement the new() constructor.
macro_rules! impl_new_constructor {
    ($struct_name:ident, $exchange:ident, $default_url:expr, $handler:expr, $translator:expr) => {
        impl $struct_name {
            /// Creates a websocket client.
            ///
            /// # Arguments
            ///
            /// * `tx` - The sending part of a channel
            /// * `url` - Optional server url, usually you don't need specify it
            pub async fn new(tx: std::sync::mpsc::Sender<String>, url: Option<&str>) -> Self {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: WSClientInternal::connect($exchange, real_url, $handler, tx).await,
                    translator: $translator,
                }
            }
        }
    };
}

/// Implement the WSClient trait.
macro_rules! impl_ws_client_trait {
    ($struct_name:ident) => {
        #[async_trait]
        impl WSClient for $struct_name {
            async fn subscribe_trade(&self, symbols: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, symbols).await
            }

            async fn subscribe_orderbook(&self, symbols: &[String]) {
                <$struct_name as OrderBook>::subscribe_orderbook(self, symbols).await
            }

            async fn subscribe_orderbook_topk(&self, symbols: &[String]) {
                <$struct_name as OrderBookTopK>::subscribe_orderbook_topk(self, symbols).await
            }

            async fn subscribe_l3_orderbook(&self, symbols: &[String]) {
                <$struct_name as Level3OrderBook>::subscribe_l3_orderbook(self, symbols).await
            }

            async fn subscribe_ticker(&self, symbols: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, symbols).await
            }

            async fn subscribe_bbo(&self, symbols: &[String]) {
                <$struct_name as BBO>::subscribe_bbo(self, symbols).await
            }

            async fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
                <$struct_name as Candlestick>::subscribe_candlestick(self, symbol_interval_list)
                    .await
            }

            async fn subscribe(&self, topics: &[(String, String)]) {
                let commands = self.translator.translate_to_commands(true, topics);
                self.client.send(&commands).await;
            }

            async fn unsubscribe(&self, topics: &[(String, String)]) {
                let commands = self.translator.translate_to_commands(false, topics);
                self.client.send(&commands).await;
            }

            async fn send(&self, commands: &[String]) {
                self.client.send(commands).await;
            }

            async fn run(&self) {
                self.client.run().await;
            }

            fn close(&self) {
                self.client.close();
            }
        }
    };
}
