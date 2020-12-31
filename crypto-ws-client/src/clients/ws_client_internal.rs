use super::utils::connect_with_retry;

use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::{
    collections::HashSet,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use flate2::read::{DeflateDecoder, GzDecoder};
use log::*;
use tungstenite::{client::AutoStream, Error, Message, WebSocket};

pub(super) enum MiscMessage {
    WebSocket(Message), // WebSocket message that needs to be sent to the server
    Reconnect,          // Needs to reconnect
    Misc,               // Misc message
    Normal,             // Normal message will be passed to on_msg
}

pub(super) struct WSClientInternal<'a> {
    exchange: String, // Eexchange name
    url: String,      // Websocket base url
    ws_stream: WebSocket<AutoStream>,
    channels: HashSet<String>,            // subscribed channels
    on_msg: Box<dyn FnMut(String) + 'a>,  // user defined message callback
    on_misc_msg: fn(&str) -> MiscMessage, // handle misc messages
    serialize_command: fn(&[String], bool) -> Vec<String>,

    // for ping thread
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl<'a> WSClientInternal<'a> {
    pub fn new(
        exchange: &str,
        url: &str,
        on_msg: Box<dyn FnMut(String) + 'a>,
        on_misc_msg: fn(&str) -> MiscMessage,
        serialize_command: fn(&[String], bool) -> Vec<String>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>();
        let ws_stream = connect_with_retry(url);
        WSClientInternal {
            exchange: exchange.to_string(),
            url: url.to_string(),
            ws_stream,
            on_msg,
            on_misc_msg,
            channels: HashSet::new(),
            serialize_command,
            sender,
            receiver,
        }
    }

    pub fn subscribe(&mut self, channels: &[String]) {
        let mut diff = Vec::<String>::new();
        for ch in channels.iter() {
            if self.channels.insert(ch.clone()) {
                diff.push(ch.clone());
            }
        }
        if !diff.is_empty() {
            let commands = (self.serialize_command)(channels, true);
            commands.into_iter().for_each(|command| {
                let res = self.ws_stream.write_message(Message::Text(command));
                if let Err(err) = res {
                    error!("{}", err);
                }
            });
        }
    }

    pub fn unsubscribe(&mut self, channels: &[String]) {
        let mut diff = Vec::<String>::new();
        for ch in channels.iter() {
            if self.channels.remove(ch) {
                diff.push(ch.clone());
            }
        }
        if !diff.is_empty() {
            let commands = (self.serialize_command)(channels, false);
            commands.into_iter().for_each(|command| {
                let res = self.ws_stream.write_message(Message::Text(command));
                if let Err(err) = res {
                    error!("{}", err);
                }
            });
        }
    }

    // reconnect and subscribe all channels
    fn reconnect(&mut self) {
        warn!("Reconnecting to {}", &self.url);
        self.ws_stream = connect_with_retry(&self.url);
        let channels = self
            .channels
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !channels.is_empty() {
            let commands = (self.serialize_command)(&channels, true);
            commands.into_iter().for_each(|command| {
                let resp = self.ws_stream.write_message(Message::Text(command));
                if let Err(err) = resp {
                    error!("{}", err);
                }
            });
        }
    }

    // Handle a text msg from Message::Text or Message::Binary
    fn handle_msg(&mut self, txt: &str) {
        if txt.contains("error") {
            error!("{} from {}", txt, self.url);
            return;
        }

        match (self.on_misc_msg)(txt) {
            MiscMessage::Misc => (),
            MiscMessage::Reconnect => {
                self.reconnect();
            }
            MiscMessage::WebSocket(ws_msg) => {
                if let Err(err) = self.ws_stream.write_message(ws_msg) {
                    error!("{}", err);
                }
            }
            MiscMessage::Normal => {
                if self.exchange == super::mxc::EXCHANGE_NAME
                    && self.url.as_str() == super::mxc::SPOT_WEBSOCKET_URL
                {
                    // special logic for MXC Spot
                    match txt.strip_prefix("42") {
                        Some(msg) => (self.on_msg)(msg.to_string()),
                        None => {
                            if txt == "1" {
                                // disconnect
                                self.reconnect();
                            }
                            warn!("Received {} from {}", txt, self.url);
                        }
                    }
                } else {
                    (self.on_msg)(txt.to_string());
                }
            }
        }
    }

    pub fn run(&mut self, duration: Option<u64>) {
        // start the ping thread
        WSClientInternal::auto_ping(
            self.exchange.to_string(),
            self.url.to_string(),
            self.sender.clone(),
        );

        let now = Instant::now();
        loop {
            let resp = self.ws_stream.read_message();
            match resp {
                Ok(msg) => match msg {
                    Message::Text(txt) => self.handle_msg(&txt),
                    Message::Binary(binary) => {
                        let mut txt = String::new();
                        let resp = if self.exchange == super::huobi::EXCHANGE_NAME {
                            let mut decoder = GzDecoder::new(&binary[..]);
                            decoder.read_to_string(&mut txt)
                        } else if self.exchange == super::okex::EXCHANGE_NAME {
                            let mut decoder = DeflateDecoder::new(&binary[..]);
                            decoder.read_to_string(&mut txt)
                        } else {
                            panic!("Unknown binary format from {}", self.url)
                        };

                        match resp {
                            Ok(_) => self.handle_msg(&txt),
                            Err(err) => error!("Decompression failed, {}", err),
                        }
                    }
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
                Err(err) => match err {
                    Error::ConnectionClosed => {
                        warn!("tungstenite::Error::ConnectionClosed");
                        self.reconnect();
                    }
                    _ => error!("{}", err),
                },
            }

            // Exchange specific ping logic
            if let Ok(ping) = self.receiver.try_recv() {
                let res = self.ws_stream.write_message(ping);
                if let Err(err) = res {
                    error!("{}, {}", err, self.url);
                }
            }

            if let Some(seconds) = duration {
                if now.elapsed() > Duration::from_secs(seconds) {
                    break;
                }
            }
        }
    }

    pub fn close(&mut self) {
        let res = self.ws_stream.close(None);
        if let Err(err) = res {
            error!("{}", err);
        }
    }

    // Send ping per interval
    fn auto_ping(exchange: String, url: String, sender: Sender<Message>) {
        thread::spawn(move || {
            loop {
                let ping_msg = match exchange.as_str() {
                    super::mxc::EXCHANGE_NAME => {
                        if url.as_str() == super::mxc::SPOT_WEBSOCKET_URL {
                            // ping per 5 seconds
                            Some((5, Message::Text("2".to_string()))) // socket.io ping
                        } else if url.as_str() == super::mxc::SWAP_WEBSOCKET_URL {
                            // ping per 10 seconds
                            Some((10, Message::Text(r#"{"method":"ping"}"#.to_string())))
                        } else {
                            None
                        }
                    }
                    super::bitmex::EXCHANGE_NAME => Some((5, Message::Text("ping".to_string()))),
                    _ => None,
                };

                match ping_msg {
                    Some(msg) => {
                        let _ = sender.send(msg.1);
                        thread::sleep(Duration::from_secs(msg.0));
                    }
                    None => return,
                }
            }
        });
    }
}

/// Define exchange specific client.
macro_rules! define_client {
    ($struct_name:ident, $exchange:ident, $default_url:ident, $serialize_command:ident, $on_misc_msg:ident) => {
        impl<'a> WSClient<'a> for $struct_name<'a> {
            fn new(on_msg: Box<dyn FnMut(String) + 'a>, url: Option<&str>) -> $struct_name<'a> {
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
                        $serialize_command,
                    ),
                }
            }

            fn subscribe(&mut self, channels: &[String]) {
                self.client.subscribe(channels);
            }

            fn unsubscribe(&mut self, channels: &[String]) {
                self.client.unsubscribe(channels);
            }

            fn run(&mut self, duration: Option<u64>) {
                self.client.run(duration);
            }

            fn close(&mut self) {
                self.client.close();
            }
        }
    };
}
