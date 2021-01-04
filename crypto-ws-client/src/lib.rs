//! A versatile websocket client that supports many cryptocurrency exchanges.
//!
//! ## Example
//!
//! ```
//! use crypto_ws_client::{BinanceSpotWSClient, WSClient};
//!
//! let mut ws_client = BinanceSpotWSClient::new(Box::new(|msg| println!("{}", msg)), None);
//! let channels = vec!["btcusdt@aggTrade".to_string(), "btcusdt@depth".to_string(),];
//! ws_client.subscribe(&channels);
//! ws_client.run(Some(2)); // run for 2 seconds
//! ```
//! ## High Level APIs
//! The following APIs are high-level APIs with ease of use:
//!
//! * subscribe_trade(&mut self, pairs: &[String])
//! * subscribe_ticker(&mut self, pairs: &[String])
//! * subscribe_bbo(&mut self, pairs: &[String])
//!
//! They are easier to use and cover mostly used scenarios.
//!
//! ## Low Level APIs
//!
//! Sometimes high-level APIs can NOT meet our needs, this package provides two low-level APIs:
//!
//! * subscribe(&mut self, raw_channels: &[String])
//! * unsubscribe(&mut self, raw_channels: &[String])
//!
//！ A `raw_channel` can be:
//!
//! * `channel:pair`
//! * `channel`
//! * A JSON string
//!
//! Usually a `raw_channel` is composed by a `channel` and a `pair`,
//! delimited by `,`. Sometimes the `pair` is optional, for example,
//! `instrument` of BitMEX, `market.overview` of Huobi.
//!
//! If a `raw_channel` starts with `{`, which means it is a JSON string,
//! then it will be sent out directly without any parsing.
//!
//! More examples:
//!
//! * BitMEX `trade:XBTUSD`, `quote:XBTM21`, `instrument`
//! * Binance `btcusdt`, `btcusd_perp`
//! * OKEx `spot/trade:BTC-USDT`
mod clients;

pub use clients::binance::*;
pub use clients::bitfinex::*;
pub use clients::bitmex::*;
pub use clients::bitstamp::*;
pub use clients::coinbase_pro::*;
pub use clients::huobi::*;
pub use clients::kraken::*;
pub use clients::mxc::*;
pub use clients::okex::*;

/// The public interface of every WebSocket client.
pub trait WSClient<'a> {
    /// Creates a new client.
    ///
    /// # Arguments
    ///
    /// * `on_msg` - A callback function to process original JSON messages
    /// * `url` - Optional server url, usually you don't need specify it
    fn new(on_msg: Box<dyn FnMut(String) + 'a>, url: Option<&str>) -> Self;

    /// Subscribes to trade channels.
    ///
    /// A trade channel sends tick-by-tick trade data,  which is the complete
    /// copy of a market's trading information.
    ///
    /// Each exchange has its own pair formats, for example:
    ///
    /// * BitMEX `XBTUSD`, `XBTM21`
    /// * Binance `btcusdt`, `btcusd_perp`
    /// * OKEx `BTC-USDT`
    fn subscribe_trade(&mut self, pairs: &[String]);

    /// Subscribes to inremental orderbook channels.
    ///
    /// A inremental orderbook channel sends a snapshot followed by tick-by-tick updates.
    ///
    /// This function subscribes to exchange specific channels as the following:
    ///
    /// * Binance `depth@100ms`
    /// * Bitfinex `book` channel with `prec=P0`, `frec=F0` and `len=25`
    /// * BitMEX `orderBookL2_25`
    /// * Bitstamp `diff_order_book`
    /// * CoinbasePro `level2`
    /// * Huobi `depth.size_20.high_freq` for contracts, `mbp.20` for Spot
    /// * Kraken `book` with `depth=25`
    /// * MXC `depth` for Swap, `symbol` for Spot
    /// * OKEx `depth_l2_tbt`
    fn subscribe_orderbook(&mut self, pairs: &[String]);

    /// Subscribes to ticker channels.
    ///
    /// A ticker channel pushes realtime 24hr rolling window ticker messages,
    /// which contains OHLCV information.
    ///
    /// Not all exchanges have the ticker channel, for example, BitMEX,
    /// Bitstamp, MXC Spot, etc.
    fn subscribe_ticker(&mut self, pairs: &[String]);

    /// Subscribes to best bid & offer(BBO) channels.
    ///
    /// BBO represents best bid and offer, i.e., level1 orderbook.
    ///
    /// Not all exchanges have the BBO channel, calling this function with
    /// these exchanges will panic.
    ///
    /// * Binance, BitMEX, Huobi and Kraken have BBO directly.
    /// * Bitfinex uses `book` channel with `len=1` and `prec="R0"` to get BBO data.
    fn subscribe_bbo(&mut self, pairs: &[String]);

    /// Subscribes to channels, lower level API.
    ///
    /// A `raw_channel` can be:
    ///
    /// * `channel:pair`
    /// * `channel`
    /// * A JSON string
    ///
    /// Usually a `raw_channel` is composed by a `channel` and a `pair`,
    /// delimited by `,`. Sometimes the `pair` is optional, for example,
    /// `instrument` of BitMEX, `market.overview` of Huobi.
    ///
    /// If a `raw_channel` starts with `{`, which means it is a JSON string,
    /// then it will be sent out directly without any parsing.
    ///
    /// More examples:
    ///
    /// * BitMEX `trade:XBTUSD`, `quote:XBTM21`, `instrument`
    /// * Binance `btcusdt`, `btcusd_perp`
    /// * OKEx `spot/trade:BTC-USDT`
    fn subscribe(&mut self, raw_channels: &[String]);

    /// Unsubscribes from channels, lower level API.
    fn unsubscribe(&mut self, raw_channels: &[String]);

    /// Starts the infinite loop until time is up or the server closes the connection.
    ///
    /// # Arguments
    ///
    /// * `duration` - How many seconds to run, None means infinite.
    fn run(&mut self, duration: Option<u64>);
}
