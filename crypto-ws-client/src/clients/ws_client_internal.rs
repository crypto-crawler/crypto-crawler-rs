use super::utils::connect_with_retry;
use log::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use tungstenite::{client::AutoStream, Error, Message, WebSocket};

use flate2::read::GzDecoder;
use std::io::prelude::*;

pub(super) struct WSClientInternal {
    url: String, // Websocket base url
    ws_stream: WebSocket<AutoStream>,
    channels: HashSet<String>, // subscribed channels
    on_msg: fn(String),        // message callback
    serialize_command: fn(&[String], bool) -> Vec<String>,

    // For debugging only
    count: i64, // message received
}

impl WSClientInternal {
    pub fn new(
        url: &str,
        on_msg: fn(String),
        serialize_command: fn(&[String], bool) -> Vec<String>,
    ) -> Self {
        let ws_stream = connect_with_retry(url);
        WSClientInternal {
            url: url.to_string(),
            ws_stream,
            on_msg,
            channels: HashSet::new(),
            serialize_command,
            count: 0,
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
        info!("Reconnecting to {}", &self.url);
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
    fn handle_msg(&mut self, txt: String) {
        if txt.contains("error") {
            error!("{}", txt);
            return;
        }
        self.count += 1;

        let resp = serde_json::from_str::<HashMap<String, Value>>(&txt);
        if resp.is_err() {
            error!("{} is not a JSON string", txt);
            return;
        }

        let obj = resp.unwrap();
        if obj.contains_key("ping") {
            let value = obj.get("ping").unwrap();
            let mut pong_msg = HashMap::<String, &Value>::new();
            pong_msg.insert("pong".to_string(), value);
            let response = Message::Text(serde_json::to_string(&pong_msg).unwrap());
            if let Err(err) = self.ws_stream.write_message(response) {
                error!("{}", err);
            }
            return;
        }

        (self.on_msg)(txt);
    }

    pub fn run(&mut self) {
        loop {
            let resp = self.ws_stream.read_message();
            match resp {
                Ok(msg) => match msg {
                    Message::Text(txt) => self.handle_msg(txt),
                    Message::Binary(binary) => {
                        let mut gz = GzDecoder::new(&binary[..]);
                        let mut txt = String::new();
                        match gz.read_to_string(&mut txt) {
                            Ok(_) => self.handle_msg(txt),
                            Err(err) => error!("Decompress failed, {}", err),
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
                        // need to reconnect
                        error!("Received a close frame");
                        if let Some(frame) = resp {
                            error!("code: {}, reason: {}", frame.code, frame.reason);
                        }
                    }
                },
                Err(err) => match err {
                    Error::ConnectionClosed => {
                        self.reconnect();
                    }
                    _ => error!("{}", err),
                },
            }
        }
    }

    pub fn close(&mut self) {
        let res = self.ws_stream.close(None);
        if let Err(err) = res {
            error!("{}", err);
        }
    }
}

/// Define exchange specific client.
macro_rules! define_client {
    ($struct_name:ident, $default_url:ident, $serialize_command:ident) => {
        impl WSClient for $struct_name {
            type Exchange = $struct_name;

            fn init(on_msg: fn(String), url: Option<&str>) -> $struct_name {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: WSClientInternal::new(real_url, on_msg, $serialize_command),
                }
            }

            fn subscribe(&mut self, channels: &[String]) {
                self.client.subscribe(channels);
            }

            fn unsubscribe(&mut self, channels: &[String]) {
                self.client.unsubscribe(channels);
            }

            fn run(&mut self) {
                self.client.run();
            }

            fn close(&mut self) {
                self.client.close();
            }
        }
    };
}
