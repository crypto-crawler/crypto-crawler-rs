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
    /// * Bitfinex uses `book` channel with `len` set to 1 to get BBO data.
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
