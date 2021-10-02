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

pub(super) trait Candlestick {
    /// Subscribes to candlestick channels which send OHLCV messages.
    ///
    /// `interval` specifies the interval of candlesticks in second unit.
    fn subscribe_candlestick(&self, pairs: &[String], interval: u32);
}

macro_rules! impl_trait {
    ($trait_name:ident, $struct_name:ident, $method_name:ident, $channel_name:expr, $to_raw_channel: ident) => {
        impl<'a> $trait_name for $struct_name<'a> {
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
        impl<'a> Candlestick for $struct_name<'a> {
            fn subscribe_candlestick(&self, pairs: &[String], interval: u32) {
                let raw_channels: Vec<String> = pairs
                    .iter()
                    .map(|pair| to_candlestick_raw_channel(pair, interval))
                    .collect();
                self.client.subscribe(&raw_channels);
            }
        }
    };
}
