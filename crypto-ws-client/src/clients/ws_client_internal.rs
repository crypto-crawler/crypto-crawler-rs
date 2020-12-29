use super::utils::connect_with_retry;
use log::*;
use std::collections::HashSet;
use tungstenite::{client::AutoStream, Error, Message, WebSocket};

pub(super) struct WSClientInternal {
    url: String, // Websocket base url
    ws_stream: WebSocket<AutoStream>,
    channels: HashSet<String>, // subscribed channels
    on_msg: fn(String),        // message callback
    serialize_command: fn(&[String], bool) -> String,
}

impl WSClientInternal {
    pub fn new(
        url: &str,
        on_msg: fn(String),
        serialize_command: fn(&[String], bool) -> String,
    ) -> Self {
        let ws_stream = connect_with_retry(url);
        WSClientInternal {
            url: url.to_string(),
            ws_stream,
            on_msg,
            channels: HashSet::new(),
            serialize_command,
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
            let serialized = (self.serialize_command)(channels, true);
            let res = self.ws_stream.write_message(Message::Text(serialized));
            if let Err(err) = res {
                error!("{}", err);
            }
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
            let serialized = (self.serialize_command)(channels, false);
            let res = self.ws_stream.write_message(Message::Text(serialized));
            if let Err(err) = res {
                error!("{}", err);
            }
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
            let command = (self.serialize_command)(&channels, true);
            let resp = self.ws_stream.write_message(Message::Text(command));
            if let Err(err) = resp {
                error!("{}", err);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let resp = self.ws_stream.read_message();
            match resp {
                Ok(msg) => match msg {
                    Message::Text(txt) => {
                        if txt.contains("error") {
                            error!("{}", txt)
                        } else {
                            (self.on_msg)(txt);
                        }
                    }
                    Message::Binary(binary) => {
                        // ws_stream.write_message(msg);
                    }
                    Message::Ping(_) => {}
                    Message::Pong(_) => {}
                    Message::Close(resp) => {
                        // need to reconnect
                        trace!("Received a close frame");
                        if let Some(frame) = resp {
                            trace!("code: {}, reason: {}", frame.code, frame.reason);
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
