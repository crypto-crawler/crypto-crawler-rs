use core::panic;

// tick-by-tick trade
pub(super) trait Trade {
    fn subscribe_trade(&mut self, pairs: &[String]);
}

// 24hr rolling window ticker
pub(super) trait Ticker {
    fn subscribe_ticker(&mut self, pairs: &[String]);
}

// Best Bid & Offer
pub(super) trait BBO {
    fn subscribe_bbo(&mut self, pairs: &[String]);
}

// An orderbook snapshot followed by realtime updates.
pub(super) trait OrderBook {
    fn subscribe_orderbook(&mut self, pairs: &[String]);
}

pub(super) trait OrderBookSnapshot {
    /// Subscribes to level2 orderbook snapshot channels.
    fn subscribe_orderbook_snapshot(&mut self, pairs: &[String]);
}

pub trait Candlestick {
    /// Subscribes to candlestick channels.
    fn subscribe_candlestick(&mut self, pairs: &[String], interval: u32);
}

macro_rules! impl_trait {
    ($trait_name:ident, $struct_name:ident, $method_name:ident, $channel_name:expr, $to_raw_channel: ident) => {
        impl<'a> $trait_name for $struct_name<'a> {
            fn $method_name(&mut self, pairs: &[String]) {
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

pub(super) fn interval_to_string(interval: u32, exchange: Option<&'static str>) -> String {
    let result = if interval < 60 {
        panic!("interval can NOT be less than 60 seconds")
    } else if interval < 3600 {
        (interval / 60, 'm')
    } else if interval < 3600 * 24 {
        (interval / 3600, 'h')
    } else if interval < 3600 * 24 * 7 {
        (interval / (3600 * 24), 'd')
    } else if interval <= 3600 * 24 * 7 * 4 {
        (interval / (3600 * 24 * 7), 'w')
    } else {
        panic!("interval greater than 4 weeks is not supported")
    };

    let suffix = match exchange {
        Some(ex) => {
            if ex == "Huobi" {
                match result.1 {
                    'm' => "min".to_string(),
                    'h' => "hour".to_string(),
                    'd' => "day".to_string(),
                    'w' => "week".to_string(),
                    _ => panic!("Unsupported interval {}", interval),
                }
            } else {
                result.1.to_string()
            }
        }
        None => result.1.to_string(),
    };

    let mut ret = result.0.to_string();
    ret.push_str(suffix.as_str());
    ret
}

macro_rules! impl_candlestick {
    ($struct_name:ident) => {
        impl<'a> Candlestick for $struct_name<'a> {
            fn subscribe_candlestick(&mut self, pairs: &[String], interval: u32) {
                let raw_channels: Vec<String> = pairs
                    .iter()
                    .map(|pair| to_candlestick_raw_channel(pair, interval))
                    .collect();
                self.client.subscribe(&raw_channels);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::interval_to_string;

    #[test]
    #[should_panic]
    fn test_less_than_60s() {
        interval_to_string(59, None);
    }

    #[test]
    #[should_panic]
    fn test_greater_than_4week() {
        interval_to_string(3600 * 24 * 29, None);
    }

    #[test]
    fn test_normal_interval() {
        assert_eq!("1m".to_string(), interval_to_string(60, None));
        assert_eq!("3m".to_string(), interval_to_string(180, None));
        assert_eq!("5m".to_string(), interval_to_string(300, None));
        assert_eq!("30m".to_string(), interval_to_string(1800, None));
        assert_eq!("1h".to_string(), interval_to_string(3600, None));
        assert_eq!("1hour".to_string(), interval_to_string(3600, Some("Huobi")));
        assert_eq!("2h".to_string(), interval_to_string(7200, None));
        assert_eq!("4h".to_string(), interval_to_string(3600 * 4, None));
        assert_eq!("1d".to_string(), interval_to_string(3600 * 24, None));
        assert_eq!(
            "1day".to_string(),
            interval_to_string(3600 * 24, Some("Huobi"))
        );
        assert_eq!("6d".to_string(), interval_to_string(3600 * 24 * 6, None));
        assert_eq!("1w".to_string(), interval_to_string(3600 * 24 * 7, None));
        assert_eq!("2w".to_string(), interval_to_string(3600 * 24 * 14, None));
        assert_eq!("4w".to_string(), interval_to_string(3600 * 24 * 28, None));
    }
}
