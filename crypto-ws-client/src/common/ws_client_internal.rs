use std::{
    io::prelude::*,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    },
    time::Duration,
};

use flate2::read::{DeflateDecoder, GzDecoder};
use log::*;
use tokio_tungstenite::tungstenite::Message;

use crate::common::message_handler::{MessageHandler, MiscMessage};

// `WSClientInternal` should be Sync + Send so that it can be put into Arc directly.
pub(crate) struct WSClientInternal<H: MessageHandler> {
    exchange: &'static str, // Eexchange name
    pub(crate) url: String, // Websocket base url
    // pass parameters to run()
    #[allow(clippy::type_complexity)]
    params_rx: std::sync::Mutex<
        tokio::sync::oneshot::Receiver<(
            H,
            tokio::sync::mpsc::Receiver<Message>,
            std::sync::mpsc::Sender<String>,
        )>,
    >,
    command_tx: tokio::sync::mpsc::Sender<Message>,
}

impl<H: MessageHandler> WSClientInternal<H> {
    pub async fn connect(
        exchange: &'static str,
        url: &str,
        handler: H,
        uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
        tx: std::sync::mpsc::Sender<String>,
    ) -> Self {
        // A channel to send parameters to run()
        let (params_tx, params_rx) = tokio::sync::oneshot::channel::<(
            H,
            tokio::sync::mpsc::Receiver<Message>,
            std::sync::mpsc::Sender<String>,
        )>();
        
        let url = format!("{}@-exchange-@{}", &url, &exchange);
        let (message_rx, command_tx) = super::connect_async::connect_async(url.as_str(), uplink_limit)
            .await
            .expect("Failed to connect to websocket");
        let _ = params_tx.send((handler, message_rx, tx));

        WSClientInternal {
            exchange,
            url: url.to_string(),
            params_rx: std::sync::Mutex::new(params_rx),
            command_tx,
        }
    }

    fn get_send_interval_ms(&self) -> Option<u64> {
        match self.exchange {
            "binance" => Some(100), // WebSocket connections have a limit of 10 incoming messages per second
            "kucoin" => Some(100),  //  Message limit sent to the server: 100 per 10 seconds
            _ => None,
        }
    }

    pub async fn send(&self, commands: &[String]) {
        for command in commands {
            debug!("{}", command);
            if self
                .command_tx
                .send(Message::Text(command.to_string()))
                .await
                .is_err()
            {
                break; // break the loop if there is no receiver
            }
            if let Some(interval) = self.get_send_interval_ms() {
                std::thread::sleep(Duration::from_millis(interval));
            }
        }
    }

    pub async fn run(&self) {
        let (mut handler, mut message_rx, tx) = {
            let mut guard = self.params_rx.lock().unwrap();
            guard.try_recv().unwrap()
        };

        let num_unanswered_ping = Arc::new(AtomicIsize::new(0)); // for debug only
        if let Some((msg, interval)) = handler.get_ping_msg_and_interval() {
            // send heartbeat periodically
            let command_tx_clone = self.command_tx.clone();
            let num_unanswered_ping_clone = num_unanswered_ping.clone();
            tokio::task::spawn(async move {
                let mut timer = {
                    let duration = Duration::from_secs(interval / 2 + 1);
                    tokio::time::interval(duration)
                };
                loop {
                    let now = timer.tick().await;
                    debug!("{:?} sending ping {}", now, msg.to_text().unwrap());
                    if let Err(err) = command_tx_clone.send(msg.clone()).await {
                        error!("Error sending ping {}", err);
                    } else {
                        num_unanswered_ping_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }
            });
        }

        while let Some(msg) = message_rx.recv().await {
            let txt = match msg {
                Message::Text(txt) => Some(txt),
                Message::Binary(binary) => {
                    let mut txt = String::new();
                    let resp = match self.exchange {
                        crate::clients::huobi::EXCHANGE_NAME
                        | crate::clients::binance::EXCHANGE_NAME
                        | "bitget"
                        | "bitz" => {
                            let mut decoder = GzDecoder::new(&binary[..]);
                            decoder.read_to_string(&mut txt)
                        }
                        crate::clients::okx::EXCHANGE_NAME => {
                            let mut decoder = DeflateDecoder::new(&binary[..]);
                            decoder.read_to_string(&mut txt)
                        }
                        _ => {
                            panic!("Unknown binary format from {}", self.url);
                        }
                    };

                    match resp {
                        Ok(_) => Some(txt),
                        Err(err) => {
                            error!("Decompression failed, {}", err);
                            None
                        }
                    }
                }
                Message::Ping(resp) => {
                    // binance server will send a ping frame every 3 or 5 minutes
                    debug!(
                        "Received a ping frame: {} from {}",
                        std::str::from_utf8(&resp).unwrap(),
                        self.url,
                    );
                    if self.exchange == "binance" {
                        // send a pong frame
                        debug!("Sending a pong frame to {}", self.url);
                        _ = self.command_tx.send(Message::Pong(Vec::new())).await;
                    }
                    None
                }
                Message::Pong(resp) => {
                    num_unanswered_ping.store(0, Ordering::Release);
                    debug!(
                        "Received a pong frame: {} from {}, reset num_unanswered_ping to {}",
                        std::str::from_utf8(&resp).unwrap(),
                        self.exchange,
                        num_unanswered_ping.load(Ordering::Acquire)
                    );
                    None
                }
                Message::Frame(_) => todo!(),
                Message::Close(resp) => {
                    match resp {
                        Some(frame) => {
                            warn!(
                                "Received a CloseFrame: code: {}, reason: {} from {}",
                                frame.code, frame.reason, self.url
                            );
                        }
                        None => warn!("Received a close message without CloseFrame"),
                    }
                    // break;
                    warn!("Received a CloseFrame"); //fail fast so that pm2 can restart the process
                    // panic!("Received a CloseFrame"); //fail fast so that pm2 can restart the process
                    None
                }
            };

            if let Some(txt) = txt {
                let txt = txt.as_str().trim().to_string();
                match handler.handle_message(&txt) {
                    MiscMessage::Normal => {
                        // the receiver might get dropped earlier than this loop
                        if tx.send(txt).is_err() {
                            break; // break the loop if there is no receiver
                        }
                    }
                    MiscMessage::Mutated(txt) => _ = tx.send(txt),
                    MiscMessage::WebSocket(ws_msg) => _ = self.command_tx.send(ws_msg).await,
                    MiscMessage::Pong => {
                        num_unanswered_ping.store(0, Ordering::Release);
                        debug!(
                            "Received {} from {}, reset num_unanswered_ping to {}",
                            txt,
                            self.exchange,
                            num_unanswered_ping.load(Ordering::Acquire)
                        );
                    }
                    MiscMessage::Reconnect => break, // fail fast, pm2 will restart, restart is reconnect
                    MiscMessage::Other => (),        // ignore
                }
            }
        }
    }

    pub fn close(&self) {
        // close the websocket connection and break the while loop in run()
        _ = self.command_tx.send(Message::Close(None));
    }
}
