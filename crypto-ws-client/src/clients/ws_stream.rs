use std::sync::{Arc, Mutex};

use log::error;
use tungstenite::{client::AutoStream, Error, Message, WebSocket};

// A thread-safe wrapper of WebSocket<AutoStream>
#[derive(Clone)]
pub(super) struct WebSocketStream {
    stream: Arc<Mutex<WebSocket<AutoStream>>>,
}

impl WebSocketStream {
    pub fn new(stream: WebSocket<AutoStream>) -> Self {
        WebSocketStream {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub fn read_message(&self) -> Result<Message, Error> {
        let mut guard = self.stream.lock().unwrap();
        guard.read_message()
    }

    pub fn write_message(&self, msg: Message) {
        let mut guard = self.stream.lock().unwrap();
        let res = guard.write_message(msg);
        if let Err(err) = res {
            error!("{}", err);
        }
    }
}

impl Drop for WebSocketStream {
    fn drop(&mut self) {
        let mut guard = self.stream.lock().unwrap();
        if let Err(err) = guard.close(None) {
            error!("{}", err);
        }
    }
}
