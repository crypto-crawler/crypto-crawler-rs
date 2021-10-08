// tick-by-tick trade
pub(super) trait Trade {
    fn subscribe_trade(&self, pairs: &[String]);
}

// 24hr rolling window ticker
pub(super) trait Ticker {
    fn subscribe_ticker(&self, pairs: &[String]);
}

// Best Bid & Offer
#[allow(clippy::upper_case_acronyms)]
pub(super) trait BBO {
    fn subscribe_bbo(&self, pairs: &[String]);
}

// An orderbook snapshot followed by realtime updates.
pub(super) trait OrderBook {
    fn subscribe_orderbook(&self, pairs: &[String]);
}

pub(super) trait OrderBookTopK {
    /// Subscribes to level2 orderbook top-k snapshot channels.
    fn subscribe_orderbook_topk(&self, pairs: &[String]);
}

/// Level3 orderbook data.
pub(super) trait Level3OrderBook {
    /// Subscribes to level3 orderebook channels.
    ///
    /// The level3 orderbook is the orginal orderbook of an exchange, it is
    /// non-aggregated by price level and updated tick-by-tick.
    fn subscribe_l3_orderbook(&self, symbols: &[String]);
}

pub(super) trait Candlestick {
    /// Subscribes to candlestick channels which send OHLCV messages.
    ///
    /// `symbol_interval_list` is a list of symbols and intervals of candlesticks in seconds.
    fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]);
}

macro_rules! impl_trait {
    ($trait_name:ident, $struct_name:ident, $method_name:ident, $channel_name:expr, $to_raw_channel: ident) => {
        impl $trait_name for $struct_name {
            fn $method_name(&self, pairs: &[String]) {
                let pair_to_raw_channel = |pair: &String| $to_raw_channel($channel_name, pair);

                let channels = pairs
                    .iter()
                    .map(pair_to_raw_channel)
                    .collect::<Vec<String>>();
                self.client.subscribe(&channels);
            }
        }
    };
}

macro_rules! impl_candlestick {
    ($struct_name:ident) => {
        impl Candlestick for $struct_name {
            fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
                let raw_channels: Vec<String> = symbol_interval_list
                    .iter()
                    .map(|(symbol, interval)| to_candlestick_raw_channel(&symbol, *interval))
                    .collect();
                self.client.subscribe(&raw_channels);
            }
        }
    };
}

macro_rules! panic_l3_orderbook {
    ($struct_name:ident) => {
        impl Level3OrderBook for $struct_name {
            fn subscribe_l3_orderbook(&self, _symbols: &[String]) {
                panic!("{} does NOT have level3 websocket channel", EXCHANGE_NAME);
            }
        }
    };
}
