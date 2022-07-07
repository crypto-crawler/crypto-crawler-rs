use fast_socks5::client::{Config, Socks5Stream};
use futures_util::{SinkExt, StreamExt};
use governor::{Quota, RateLimiter};
use log::*;
use nonzero_ext::*;
use reqwest::Url;
use tokio::time::interval;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;
use std::{env, rc::Rc};
use std::num::NonZeroU32;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::mpsc::{Receiver, Sender},
};
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};

/// Wraps a websocket client inside an event loop, returns a message_rx to receive messages and
/// a command_tx to send commands to the websocket server.
///
/// To close the websocket connection, send a `Message::Close` message to the command_tx.
///
/// `limit`, max number of uplink messsages, for example, 100 per 10 seconds
pub async fn connect_async(
    url: &str,
    uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
) -> Result<(Receiver<Message>, Sender<Message>), Error> {
    let (url, exchange) = {
      let data: Vec<String> = url.to_string().split("@-exchange-@").map(|s| s.to_string()).collect();
      if data.len() == 2 {
          (data[0].to_owned(), data[1].to_owned())
      } else {
          (data[0].to_owned(), "N".to_string())
      }
    };
    let (command_tx, command_rx) = tokio::sync::mpsc::channel::<Message>(1);
    let (message_tx, message_rx) = tokio::sync::mpsc::channel::<Message>(32);
    let command_tx_clone = command_tx.clone();
    let url = url.to_string();
    let mut index = 1;

    info!("linking exchange: {}", exchange);
    if let Ok(proxy_env) = env::var("https_proxy").or_else(|_| env::var("http_proxy")) {
      let url = url.to_string();
      let mut command_buf = None;
      tokio::task::spawn(async move {
        let mut command_rx = RefCell::new(command_rx);
        let command_tx_clone = command_tx_clone.clone();
        while 5 > index {
          let proxy_url = Url::parse(&proxy_env).unwrap();
          let proxy_scheme = proxy_url.scheme().to_lowercase();
          if proxy_scheme.as_str() != "socks5" {
              panic!("Unsupported proxy scheme {}", proxy_scheme);
          }
          let proxy_addr = format!(
              "{}:{}",
              proxy_url.host_str().unwrap(),
              proxy_url.port_or_known_default().unwrap()
          );
          let connect_url = Url::parse(url.as_str()).unwrap();
          let proxy_stream = Socks5Stream::connect(
              proxy_addr.to_string(),
              connect_url.host_str().unwrap().to_string(),
              connect_url.port().unwrap(),
              Config::default(),
          )
          .await
          .unwrap();
          let ret = tokio_tungstenite::client_async_tls(connect_url, proxy_stream).await;

          if let Err(e) = ret {
            error!("index:{} {}", index, e);
            index += 1;
            continue;
          }
          index = 1;
          let (ws_stream, _) = ret.unwrap();
          let message_tx = message_tx.clone();
          let command_rx = command_rx.get_mut();
          if let Some(command) = command_buf {
            if let Err(msg) = command_tx_clone.send(command).await {
              error!("command err: {}", msg);
              break;
            };
          }
          command_buf = connect_async_internal(ws_stream, uplink_limit, command_rx, message_tx).await;
          warn!("link out; exchange: {}\nurl: {}", exchange, url);
        }
        error!("link out; exchange: {}\nurl: {}", exchange, url);
      });
    } else {
      let mut command_buf = None;
      tokio::task::spawn(async move {
        let mut command_rx = RefCell::new(command_rx);
        let command_tx_clone = command_tx_clone.clone();
        while 5 > index {
          let ret = tokio_tungstenite::connect_async(url.as_str()).await;
          if let Err(e) = ret {
            warn!("exchange: {} index:{} - msg:{}", 
              exchange, 
              index, 
              e
            );
            index += 1;
            continue;
          }
          index = 1;
          let (ws_stream, _) = ret.unwrap();
          let message_tx = message_tx.clone();
          let command_rx = command_rx.get_mut();

          if let Some(command) = command_buf {
            if let Err(msg) = command_tx_clone.send(command).await {
              info!("command err: {}", msg);
              break;
            };
          }

          command_buf = connect_async_internal(ws_stream, uplink_limit, command_rx, message_tx).await;
          warn!("link out; exchange: {}\nurl: {}", exchange, url);
        }
        error!("link out; exchange: {}\nurl: {}", exchange, url);
      });
    }
    Ok((message_rx, command_tx.clone()))
}

async fn connect_async_internal<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
    ws_stream: WebSocketStream<MaybeTlsStream<S>>,
    uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
    command_rx: &mut Receiver<Message>,
    message_tx: Sender<Message>
) -> Option<Message> {

    let mut message = None;
    let (mut write, mut read) = ws_stream.split();

    let limiter = if let Some((max_burst, duration)) = uplink_limit {
        let quota = Quota::with_period(duration).unwrap().allow_burst(max_burst);
        RateLimiter::direct(quota)
    } else {
        RateLimiter::direct(Quota::per_second(nonzero!(u32::max_value())))
    };

    let mut clock = interval(Duration::from_secs(5));
    let mut data_zone_num = 1;
    clock.reset();
    let mut flag = true;

    loop {
        tokio::select! {
          command = command_rx.recv() => {
            match command {
              Some(command) => {
                info!("commad send: {}", command);
                if let None = message {
                  message = Some(command.clone())
                }
                match command {
                  Message::Close(_) => {
                    break; // close the connection and break the loop
                  }
                  _ => {
                    limiter.until_ready().await;
                    if let Err(err) =write.send(command).await {
                      error!("Failed to send, error: {}", err);
                    }
                  }
                }
              }
              None => {
                debug!("command_rx closed");
                break;
              }
            }
          },
          msg = read.next() => match msg {
            Some(Ok(msg)) => {
              clock.reset();
              if let Message::Close(_v) = msg {
                warn!("Received a CloseFrame");
                break;
              }
              let _ = message_tx.send(msg).await;
            }
            Some(Err(err)) => {
              error!("Failed to read, error: {}", err);
              break;
            }
            None => {
              debug!("message_tx closed");
              break;
            }
          },
          _ = clock.tick() => {
            warn!("recv data timeout {}", data_zone_num);
            if data_zone_num > 3 {
              break;
            } else {
              data_zone_num += 1;
            }
          }
        };
    }
    _ = write.send(Message::Close(None)).await;
    message
}
