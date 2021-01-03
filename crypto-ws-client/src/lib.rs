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

    /// Subscribe trade channels.
    ///
    /// Each exchange has its own pair formats, for example:
    ///
    /// * BitMEX `XBTUSD`, `XBTM21`
    /// * Binance `btcusdt`, `btcusd_perp`
    /// * OKEx `BTC-USDT`
    fn subscribe_trade(&mut self, pairs: &[String]);

    /// Subscribes channels.
    ///
    /// Usually a `raw_channel` is composed by a `channel` and a `pair`,
    /// delimited by `,`. Sometimes the `pair` is optional, for example,
    /// `instrument` of BitMEX, `market.overview` of Huobi.
    ///
    /// More examples:
    ///
    /// * BitMEX `trade:XBTUSD`, `quote:XBTM21`, `instrument`
    /// * Binance `btcusdt`, `btcusd_perp`
    /// * OKEx `spot/trade:BTC-USDT`
    fn subscribe(&mut self, raw_channels: &[String]);

    /// Unsubscribes channels.
    fn unsubscribe(&mut self, raw_channels: &[String]);

    /// Starts the infinite loop until time is up or the server closes the connection.
    ///
    /// # Arguments
    ///
    /// * `duration` - How many seconds to run, None means infinite.
    fn run(&mut self, duration: Option<u64>);
}
