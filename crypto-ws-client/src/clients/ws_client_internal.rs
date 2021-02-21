use super::utils::connect_with_retry;
use std::{
    collections::HashSet,
    io::prelude::*,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use flate2::read::{DeflateDecoder, GzDecoder};
use log::*;
use tungstenite::{client::AutoStream, error::ProtocolError, Error, Message, WebSocket};

pub(super) enum MiscMessage {
    WebSocket(Message), // WebSocket message that needs to be sent to the server
    Reconnect,          // Needs to reconnect
    Misc,               // Misc message
    Normal,             // Normal message will be passed to on_msg
}

pub(super) struct WSClientInternal<'a> {
    exchange: &'static str, // Eexchange name
    pub(super) url: String, // Websocket base url
    ws_stream: Mutex<WebSocket<AutoStream>>,
    channels: Mutex<HashSet<String>>, // subscribed channels
    on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>, // user defined message callback
    on_misc_msg: fn(&str) -> MiscMessage, // handle misc messages
    // converts raw channels to subscribe/unsubscribe commands
    channels_to_commands: fn(&[String], bool) -> Vec<String>,
    should_stop: AtomicBool, // used by close() and run()
    // how often the client should send a ping, None means the client doesn't need to send
    // ping, instead the server will send ping and the client just needs to reply a pong
    ping_interval_and_msg: Option<(u64, &'static str)>,
}

impl<'a> WSClientInternal<'a> {
    pub fn new(
        exchange: &'static str,
        url: &str,
        on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>,
        on_misc_msg: fn(&str) -> MiscMessage,
        channels_to_commands: fn(&[String], bool) -> Vec<String>,
        ping_interval_and_msg: Option<(u64, &'static str)>,
    ) -> Self {
        let stream = connect_with_retry(
            url,
            if let Some(interval_and_msg) = ping_interval_and_msg {
                Some(interval_and_msg.0 / 2)
            } else {
                None
            },
        );
        WSClientInternal {
            exchange,
            url: url.to_string(),
            ws_stream: Mutex::new(stream),
            on_msg,
            on_misc_msg,
            channels: Mutex::new(HashSet::new()),
            channels_to_commands,
            should_stop: AtomicBool::new(false),
            ping_interval_and_msg,
        }
    }

    pub fn subscribe(&self, channels: &[String]) {
        self.subscribe_or_unsubscribe(channels, true);
    }

    pub fn unsubscribe(&self, channels: &[String]) {
        self.subscribe_or_unsubscribe(channels, false);
    }

    fn subscribe_or_unsubscribe(&self, channels: &[String], subscribe: bool) {
        let mut diff = Vec::<String>::new();
        {
            let mut guard = self.channels.lock().unwrap();
            for ch in channels.iter() {
                if guard.insert(ch.clone()) {
                    diff.push(ch.clone());
                }
            }
        }

        if !diff.is_empty() {
            let commands = (self.channels_to_commands)(&diff, subscribe);
            let mut ws_stream = self.ws_stream.lock().unwrap();
            commands.into_iter().for_each(|command| {
                let ret = ws_stream.write_message(Message::Text(command));
                if let Err(err) = ret {
                    error!("{}", err);
                }
            });
        }
    }

    // reconnect and subscribe all channels
    fn _reconnect(&self) {
        warn!("Reconnecting to {}", &self.url);
        {
            let mut guard = self.ws_stream.lock().unwrap();
            *guard = connect_with_retry(
                self.url.as_str(),
                if let Some(interval_and_msg) = self.ping_interval_and_msg {
                    Some(interval_and_msg.0 / 2)
                } else {
                    None
                },
            );
        }
        let channels = self
            .channels
            .lock()
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !channels.is_empty() {
            let commands = (self.channels_to_commands)(&channels, true);
            let mut ws_stream = self.ws_stream.lock().unwrap();
            commands.into_iter().for_each(|command| {
                let ret = ws_stream.write_message(Message::Text(command));
                if let Err(err) = ret {
                    error!("{}", err);
                }
            });
        }
    }

    // Handle a text msg from Message::Text or Message::Binary
    // Returns true if gets a normal message, otherwise false
    fn handle_msg(&self, txt: &str) -> bool {
        match (self.on_misc_msg)(txt) {
            MiscMessage::Misc => false,
            MiscMessage::Reconnect => {
                // self.reconnect();
                std::thread::sleep(Duration::from_secs(5));
                std::process::exit(0); // fail fast, pm2 will restart
            }
            MiscMessage::WebSocket(ws_msg) => {
                let ret = self.ws_stream.lock().unwrap().write_message(ws_msg);
                if let Err(err) = ret {
                    error!("{}", err);
                }
                false
            }
            MiscMessage::Normal => {
                if self.exchange == super::mxc::EXCHANGE_NAME
                    && self.url.as_str() == super::mxc::SPOT_WEBSOCKET_URL
                {
                    // special logic for MXC Spot
                    match txt.strip_prefix("42") {
                        Some(msg) => (self.on_msg.lock().unwrap())(msg.to_string()),
                        None => error!(
                            "{}, Not possible, should be handled by {}.on_misc_msg() previously",
                            txt, self.exchange
                        ),
                    }
                } else {
                    (self.on_msg.lock().unwrap())(txt.to_string());
                }
                true
            }
        }
    }

    pub fn run(&self, duration: Option<u64>) {
        let start_timstamp = Instant::now();
        let mut last_ping_timestamp = Instant::now();
        while !self.should_stop.load(Ordering::Acquire) {
            let resp = self.ws_stream.lock().unwrap().read_message();
            let normal = match resp {
                Ok(msg) => match msg {
                    Message::Text(txt) => self.handle_msg(&txt),
                    Message::Binary(binary) => {
                        let mut txt = String::new();
                        let resp = match self.exchange {
                            super::huobi::EXCHANGE_NAME
                            | super::binance::EXCHANGE_NAME
                            | super::bitget::EXCHANGE_NAME => {
                                let mut decoder = GzDecoder::new(&binary[..]);
                                decoder.read_to_string(&mut txt)
                            }
                            super::okex::EXCHANGE_NAME => {
                                let mut decoder = DeflateDecoder::new(&binary[..]);
                                decoder.read_to_string(&mut txt)
                            }
                            _ => {
                                error!("Unknown binary format from {}", self.url);
                                panic!("Unknown binary format from {}", self.url);
                            }
                        };

                        match resp {
                            Ok(_) => self.handle_msg(&txt),
                            Err(err) => {
                                error!("Decompression failed, {}", err);
                                false
                            }
                        }
                    }
                    Message::Ping(resp) => {
                        info!(
                            "Received a ping frame: {}",
                            std::str::from_utf8(&resp).unwrap()
                        );
                        let ret = self
                            .ws_stream
                            .lock()
                            .unwrap()
                            .write_message(Message::Pong(resp));
                        if let Err(err) = ret {
                            error!("{}", err);
                        }
                        false
                    }
                    Message::Pong(resp) => {
                        let tmp = std::str::from_utf8(&resp);
                        warn!("Received a pong frame: {}", tmp.unwrap());
                        false
                    }
                    Message::Close(resp) => {
                        match resp {
                            Some(frame) => warn!("Received a Message::Close message with a CloseFrame: code: {}, reason: {}", frame.code, frame.reason),
                            None => warn!("Received a close message without CloseFrame"),
                        }
                        false
                    }
                },
                Err(err) => {
                    match err {
                        Error::ConnectionClosed => {
                            error!("Server closed connection, exiting now...");
                            // self.reconnect();
                            std::thread::sleep(Duration::from_secs(5));
                            std::process::exit(0); // fail fast, pm2 will restart
                        }
                        Error::AlreadyClosed => {
                            error!("Impossible to happen, fix the bug in the code");
                            panic!("Impossible to happen, fix the bug in the code");
                        }
                        Error::Io(io_err) => {
                            if io_err.kind() == std::io::ErrorKind::WouldBlock {
                                info!("read_message() timeout");
                            } else {
                                error!(
                                    "I/O error thrown from read_message(): {}, {:?}",
                                    io_err,
                                    io_err.kind()
                                );
                                // self.reconnect();
                                std::thread::sleep(Duration::from_secs(5));
                                std::process::exit(0); // fail fast, pm2 will restart
                            }
                        }
                        Error::Protocol(protocol_err) => {
                            if protocol_err == ProtocolError::ResetWithoutClosingHandshake {
                                error!("ResetWithoutClosingHandshake");
                                // self.reconnect();
                                std::thread::sleep(Duration::from_secs(5));
                                std::process::exit(0); // fail fast, pm2 will restart
                            } else {
                                error!(
                                    "Protocol error thrown from read_message(): {}",
                                    protocol_err
                                );
                            }
                        }
                        _ => {
                            error!("Error thrown from read_message(): {}", err);
                            panic!("Error thrown from read_message(): {}", err);
                        }
                    }

                    false
                }
            };

            if let Some(interval_and_msg) = self.ping_interval_and_msg {
                if last_ping_timestamp.elapsed() >= Duration::from_secs(interval_and_msg.0 / 2) {
                    info!("Sending ping: {}", interval_and_msg.1);
                    // send ping
                    let ping_msg = Message::Text(interval_and_msg.1.to_string());
                    last_ping_timestamp = Instant::now();
                    if let Err(err) = self.ws_stream.lock().unwrap().write_message(ping_msg) {
                        error!("{}", err);
                    }
                }
            }

            if let Some(seconds) = duration {
                if start_timstamp.elapsed() > Duration::from_secs(seconds) && normal {
                    break;
                }
            }
        }
    }

    pub fn close(&self) {
        self.should_stop.store(true, Ordering::Release);
        let ret = self.ws_stream.lock().unwrap().close(None);
        if let Err(err) = ret {
            error!("{}", err);
        }
    }
}

/// Define exchange specific client.
macro_rules! define_client {
    ($struct_name:ident, $exchange:ident, $default_url:ident, $channels_to_commands:ident, $on_misc_msg:ident, $ping_interval:expr) => {
        impl<'a> WSClient<'a> for $struct_name<'a> {
            fn new(
                on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>,
                url: Option<&str>,
            ) -> $struct_name<'a> {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: WSClientInternal::new(
                        $exchange,
                        real_url,
                        on_msg,
                        $on_misc_msg,
                        $channels_to_commands,
                        $ping_interval,
                    ),
                }
            }

            fn subscribe_trade(&self, channels: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, channels);
            }

            fn subscribe_orderbook(&self, channels: &[String]) {
                <$struct_name as OrderBook>::subscribe_orderbook(self, channels);
            }

            fn subscribe_orderbook_snapshot(&self, channels: &[String]) {
                <$struct_name as OrderBookSnapshot>::subscribe_orderbook_snapshot(self, channels);
            }

            fn subscribe_ticker(&self, channels: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, channels);
            }

            fn subscribe_bbo(&self, channels: &[String]) {
                <$struct_name as BBO>::subscribe_bbo(self, channels);
            }

            fn subscribe_candlestick(&self, pairs: &[String], interval: u32) {
                <$struct_name as Candlestick>::subscribe_candlestick(self, pairs, interval);
            }

            fn subscribe(&self, channels: &[String]) {
                self.client.subscribe(channels);
            }

            fn unsubscribe(&self, channels: &[String]) {
                self.client.unsubscribe(channels);
            }

            fn run(&self, duration: Option<u64>) {
                self.client.run(duration);
            }

            fn close(&self) {
                self.client.close();
            }
        }
    };
}
