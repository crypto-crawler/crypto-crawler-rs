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
    /// Create a new client.
    ///
    /// # Arguments
    ///
    /// * `on_msg` - The message handler
    /// * `url` - Optional server url, usually you don't need specify it
    fn new(on_msg: Box<dyn FnMut(String) + 'a>, url: Option<&str>) -> Self;

    /// Subscribe channels.
    fn subscribe(&mut self, channels: &[String]);

    /// Unsubscribe channels.
    fn unsubscribe(&mut self, channels: &[String]);

    /// Start the infinite loop until the server closes the connection.
    ///
    /// # Arguments
    ///
    /// * `duration` - How many seconds to run, None means infinite.
    fn run(&mut self, duration: Option<u64>);
}
