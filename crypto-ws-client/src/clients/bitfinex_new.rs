use crate::{Level3OrderBook, WSClient};

use std::{
    collections::{HashMap, HashSet},
    thread,
    time::{Duration, Instant},
};

use super::{
    utils::{connect_with_retry, CHANNEL_PAIR_DELIMITER},
    ws_stream::WebSocketStream,
    Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;
use tungstenite::{Error, Message};

pub(super) const EXCHANGE_NAME: &str = "Bitfinex";

const WEBSOCKET_URL: &str = "wss://api-pub.bitfinex.com/ws/2";

/// The WebSocket client for Bitfinex, including all markets.
///
/// * WebSocket API doc: <https://docs.bitfinex.com/docs/ws-general>
/// * Spot: <https://trading.bitfinex.com/trading>
/// * Swap: <https://trading.bitfinex.com/t/BTCF0:USTF0>
/// * Funding: <https://trading.bitfinex.com/funding>
pub struct BitfinexWSClient<'a> {
    ws_stream: WebSocketStream,
    channels: HashSet<String>,             // subscribed channels
    on_msg: Box<dyn FnMut(String) + 'a>,   // user defined message callback
    channel_id_meta: HashMap<i64, String>, // CHANNEL_ID information
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    if channel.starts_with('{') {
        return channel.to_string();
    }
    let delim = channel.find(CHANNEL_PAIR_DELIMITER).unwrap();
    let ch = &channel[..delim];
    let symbol = &channel[(delim + 1)..];

    format!(
        r#"{{"event": "{}", "channel": "{}", "symbol": "{}"}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        ch,
        symbol
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|s| channel_to_command(s, subscribe))
        .collect()
}

macro_rules! impl_trait_for_bitfinex {
    ($trait_name:ident, $method_name:ident, $channel_name:expr) => {
        impl<'a> $trait_name for BitfinexWSClient<'a> {
            fn $method_name(&mut self, symbols: &[String]) {
                let symbol_to_raw_channel =
                    |symbol: &String| format!("{}:{}", $channel_name, symbol);

                let channels = symbols
                    .iter()
                    .map(symbol_to_raw_channel)
                    .collect::<Vec<String>>();
                self.subscribe(&channels);
            }
        }
    };
}

impl_trait_for_bitfinex!(Trade, subscribe_trade, "trades");
impl_trait_for_bitfinex!(Ticker, subscribe_ticker, "ticker");

impl<'a> BBO for BitfinexWSClient<'a> {
    fn subscribe_bbo(&mut self, symbols: &[String]) {
        let raw_channels = symbols
            .iter()
            .map(|symbol| {
                format!(
                    r#"{{
                "event": "subscribe",
                "channel": "book",
                "symbol": "{}",
                "prec": "R0",
                "len": 1
              }}"#,
                    symbol
                )
            })
            .collect::<Vec<String>>();

        self.subscribe(&raw_channels);
    }
}

impl<'a> OrderBook for BitfinexWSClient<'a> {
    fn subscribe_orderbook(&mut self, symbols: &[String]) {
        let raw_channels = symbols
            .iter()
            .map(|symbol| {
                format!(
                    r#"{{
                "event": "subscribe",
                "channel": "book",
                "symbol": "{}",
                "prec": "P0",
                "frec": "F0",
                "len": 25
              }}"#,
                    symbol
                )
            })
            .collect::<Vec<String>>();

        self.subscribe(&raw_channels);
    }
}

impl<'a> OrderBookSnapshot for BitfinexWSClient<'a> {
    fn subscribe_orderbook_snapshot(&mut self, _symbols: &[String]) {
        panic!("Bitfinex does NOT have orderbook snapshot channel");
    }
}

impl<'a> Level3OrderBook for BitfinexWSClient<'a> {
    fn subscribe_l3_orderbook(&mut self, symbols: &[String]) {
        let raw_channels = symbols
            .iter()
            .map(|symbol| {
                format!(
                    r#"{{
                        "event": "subscribe",
                        "channel": "book",
                        "symbol": "{}",
                        "prec": "R0",
                        "len": 250
                    }}"#,
                    symbol
                )
            })
            .collect::<Vec<String>>();

        self.subscribe(&raw_channels);
    }
}

fn to_candlestick_raw_channel(symbol: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1m",
        300 => "5m",
        900 => "15m",
        1800 => "30m",
        3600 => "1h",
        10800 => "3h",
        21600 => "6h",
        43200 => "12h",
        86400 => "1D",
        604800 => "7D",
        1209600 => "14D",
        2592000 => "1M",
        _ => panic!("Bitfinex has intervals 1m,5m,15m,30m,1h,3h,6h,12h,1D,7D,14D,1M"),
    };

    format!(
        r#"{{
            "event": "subscribe",
            "channel": "candles",
            "key": "trade:{}:{}"
        }}"#,
        interval_str, symbol
    )
}

impl<'a> Candlestick for BitfinexWSClient<'a> {
    fn subscribe_candlestick(&mut self, symbols: &[String], interval: u32) {
        let raw_channels: Vec<String> = symbols
            .iter()
            .map(|symbol| to_candlestick_raw_channel(symbol, interval))
            .collect();
        self.subscribe(&raw_channels);
    }
}

impl<'a> BitfinexWSClient<'a> {
    // reconnect and subscribe all channels
    fn reconnect(&mut self) {
        warn!("Reconnecting to {}", WEBSOCKET_URL);
        self.ws_stream = WebSocketStream::new(connect_with_retry(WEBSOCKET_URL));
        let channels = self
            .channels
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !channels.is_empty() {
            let commands = channels_to_commands(&channels, true);
            commands.into_iter().for_each(|command| {
                self.ws_stream.write_message(Message::Text(command));
            });
        }
        // avoid too frequent reconnect
        std::thread::sleep(Duration::from_secs(5));
    }

    // Handle a text msg from Message::Text or Message::Binary
    // Returns true if gets a normal message, otherwise false
    fn handle_msg(&mut self, txt: &str) -> bool {
        if txt.starts_with('{') {
            let mut obj = serde_json::from_str::<HashMap<String, Value>>(txt).unwrap();
            let event = obj.get("event").unwrap().as_str().unwrap();
            match event {
                "error" => {
                    let code = obj.get("code").unwrap().as_i64().unwrap();
                    match code {
                        10301 | 10401 => {
                            // 10301: Already subscribed
                            // 10401: Not subscribed
                            // 10000: Unknown event
                            warn!("{} from {}", txt, EXCHANGE_NAME);
                        }
                        10300 | 10400 | 10302 => {
                            // 10300, 10400:Subscription failed
                            // 10302: Unknown channel
                            // 10001: Unknown pair
                            // 10305: Reached limit of open channels
                            error!("{} from {}", txt, EXCHANGE_NAME);
                            panic!("{} from {}", txt, EXCHANGE_NAME);
                        }
                        _ => warn!("{} from {}", txt, EXCHANGE_NAME),
                    }
                }
                "info" => {
                    println!("{}", txt);
                    if let Some(_) = obj.get("version") {
                        // 1 for operative, 0 for maintenance
                        let status = obj
                            .get("platform")
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .get("status")
                            .unwrap()
                            .as_i64()
                            .unwrap();
                        if status == 0 {
                            std::thread::sleep(Duration::from_secs(15));
                        }
                    } else {
                        let code = obj.get("code").unwrap().as_i64().unwrap();
                        match code {
                            20051 => {
                                // Stop/Restart Websocket Server (please reconnect)
                                self.reconnect();
                            }
                            20060 => {
                                // Entering in Maintenance mode. Please pause any activity and resume
                                // after receiving the info message 20061 (it should take 120 seconds
                                // at most).
                                std::thread::sleep(Duration::from_secs(15));
                            }
                            20061 => {
                                // Maintenance ended. You can resume normal activity. It is advised
                                // to unsubscribe/subscribe again all channels.
                                let channels = self
                                    .channels
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>();
                                let commands = channels_to_commands(&channels, true);
                                commands.into_iter().for_each(|command| {
                                    self.ws_stream.write_message(Message::Text(command));
                                });
                            }
                            _ => info!("{} from {}", txt, EXCHANGE_NAME),
                        }
                    }
                }
                "pong" => debug!("{} from {}", txt, EXCHANGE_NAME),
                "conf" => warn!("{} from {}", txt, EXCHANGE_NAME),
                "subscribed" => {
                    let chan_id = obj.get("chanId").unwrap().as_i64().unwrap();
                    obj.remove("event");
                    obj.remove("chanId");
                    self.channel_id_meta
                        .insert(chan_id, serde_json::to_string(&obj).unwrap());
                }
                "unsubscribed" => {
                    let chan_id = obj.get("chanId").unwrap().as_i64().unwrap();
                    self.channel_id_meta.remove(&chan_id);
                }
                _ => (),
            }
            false
        } else {
            debug_assert!(txt.starts_with('['));
            // replace CHANNEL_ID with meta info
            let i = txt.find(",[").unwrap();
            let channel_id = (&txt[1..i]).parse::<i64>().unwrap();
            let channel_info = self.channel_id_meta.get(&channel_id).unwrap();
            let new_txt = format!("[{}{}", channel_info, &txt[i..]);

            (self.on_msg)(new_txt);
            true
        }
    }

    // Send ping per 30s
    fn auto_ping(ws_stream: WebSocketStream) {
        thread::spawn(move || loop {
            ws_stream.write_message(Message::Text(r#"{"event":"ping"}"#.to_string()));
            thread::sleep(Duration::from_secs(30));
        });
    }
}

impl<'a> WSClient<'a> for BitfinexWSClient<'a> {
    fn new(on_msg: Box<dyn FnMut(String) + 'a>, _url: Option<&str>) -> Self {
        let stream = connect_with_retry(WEBSOCKET_URL);
        BitfinexWSClient {
            ws_stream: WebSocketStream::new(stream),
            channels: HashSet::new(),
            on_msg,
            channel_id_meta: HashMap::new(),
        }
    }

    fn subscribe_trade(&mut self, channels: &[String]) {
        <Self as Trade>::subscribe_trade(self, channels);
    }

    fn subscribe_orderbook(&mut self, channels: &[String]) {
        <Self as OrderBook>::subscribe_orderbook(self, channels);
    }

    fn subscribe_orderbook_snapshot(&mut self, channels: &[String]) {
        <Self as OrderBookSnapshot>::subscribe_orderbook_snapshot(self, channels);
    }

    fn subscribe_ticker(&mut self, channels: &[String]) {
        <Self as Ticker>::subscribe_ticker(self, channels);
    }

    fn subscribe_bbo(&mut self, channels: &[String]) {
        <Self as BBO>::subscribe_bbo(self, channels);
    }

    fn subscribe_candlestick(&mut self, symbols: &[String], interval: u32) {
        <Self as Candlestick>::subscribe_candlestick(self, symbols, interval);
    }

    fn subscribe(&mut self, channels: &[String]) {
        let mut diff = Vec::<String>::new();
        for ch in channels.iter() {
            if self.channels.insert(ch.clone()) {
                diff.push(ch.clone());
            }
        }
        if !diff.is_empty() {
            let commands = channels_to_commands(channels, true);
            commands.into_iter().for_each(|command| {
                self.ws_stream.write_message(Message::Text(command));
            });
        }
    }

    fn unsubscribe(&mut self, channels: &[String]) {
        let mut diff = Vec::<String>::new();
        for ch in channels.iter() {
            if self.channels.remove(ch) {
                diff.push(ch.clone());
            }
        }
        if !diff.is_empty() {
            let commands = channels_to_commands(channels, false);
            commands.into_iter().for_each(|command| {
                self.ws_stream.write_message(Message::Text(command));
            });
        }
    }

    fn run(&mut self, duration: Option<u64>) {
        // start the ping thread
        Self::auto_ping(self.ws_stream.clone());

        let now = Instant::now();
        loop {
            let resp = self.ws_stream.read_message();
            let mut succeeded = false;
            match resp {
                Ok(msg) => match msg {
                    Message::Text(txt) => succeeded = self.handle_msg(&txt),
                    Message::Binary(_) => panic!("Unknown binary format from Bitfinex"),
                    Message::Ping(resp) => {
                        let tmp = std::str::from_utf8(&resp);
                        warn!("Received a ping frame: {}", tmp.unwrap());
                    }
                    Message::Pong(resp) => {
                        let tmp = std::str::from_utf8(&resp);
                        warn!("Received a pong frame: {}", tmp.unwrap());
                    }
                    Message::Close(resp) => {
                        match resp {
                            Some(frame) => warn!("Received a Message::Close message with a CloseFrame: code: {}, reason: {}", frame.code, frame.reason),
                            None => warn!("Received a close message without CloseFrame"),
                        }
                    }
                },
                Err(err) => {
                    match err {
                        Error::ConnectionClosed => {
                            warn!("tungstenite::Error::ConnectionClosed");
                            self.reconnect();
                        }
                        _ => error!("{}", err),
                    };
                }
            };

            if let Some(seconds) = duration {
                if now.elapsed() > Duration::from_secs(seconds) && succeeded {
                    break;
                }
            }
        }
    }

    fn close(&mut self) {
        self.ws_stream.close();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_spot_command() {
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            super::channel_to_command("trades:tBTCUSD", true)
        );

        assert_eq!(
            r#"{"event": "unsubscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            super::channel_to_command("trades:tBTCUSD", false)
        );
    }

    #[test]
    fn test_swap_command() {
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCF0:USTF0"}"#,
            super::channel_to_command("trades:tBTCF0:USTF0", true)
        );

        assert_eq!(
            r#"{"event": "unsubscribe", "channel": "trades", "symbol": "tBTCF0:USTF0"}"#,
            super::channel_to_command("trades:tBTCF0:USTF0", false)
        );
    }
}
