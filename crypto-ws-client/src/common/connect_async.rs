use fast_socks5::client::{Config, Socks5Stream};
use futures_util::{SinkExt, StreamExt};
use governor::{Quota, RateLimiter};
use log::*;
use nonzero_ext::*;
use reqwest::Url;
use std::{env, num::NonZeroU32};
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
    if let Ok(proxy_env) = env::var("https_proxy").or_else(|_| env::var("http_proxy")) {
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
        let connect_url = Url::parse(url).unwrap();
        let proxy_stream = Socks5Stream::connect(
            proxy_addr.to_string(),
            connect_url.host_str().unwrap().to_string(),
            connect_url.port().unwrap(),
            Config::default(),
        )
        .await
        .unwrap();
        let (ws_stream, _) = tokio_tungstenite::client_async_tls(connect_url, proxy_stream).await?;
        // replaced
        // let ret = tokio_tungstenite::connect_async(url).await;
        connect_async_internal(ws_stream, uplink_limit).await
    } else {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;

        connect_async_internal(ws_stream, uplink_limit).await
    }
}

async fn connect_async_internal<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
    ws_stream: WebSocketStream<MaybeTlsStream<S>>,
    uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
) -> Result<(Receiver<Message>, Sender<Message>), Error> {
    let (command_tx, mut command_rx) = tokio::sync::mpsc::channel::<Message>(1);
    let (message_tx, message_rx) = tokio::sync::mpsc::channel::<Message>(32);

    let (mut write, mut read) = ws_stream.split();

    let limiter = if let Some((max_burst, duration)) = uplink_limit {
        let quota = Quota::with_period(duration).unwrap().allow_burst(max_burst);
        RateLimiter::direct(quota)
    } else {
        RateLimiter::direct(Quota::per_second(nonzero!(u32::max_value())))
    };

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
              command = command_rx.recv() => {
                match command {
                  Some(command) => {
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
              }
              msg = read.next() => match msg {
                Some(Ok(msg)) => {
                  let _= message_tx.send(msg).await;
                }
                Some(Err(err)) => {
                  error!("Failed to read, error: {}", err);
                  break;
                }
                None => {
                  debug!("message_tx closed");
                  break;
                }
              }
            };
        }
        _ = write.send(Message::Close(None)).await;
    });

    Ok((message_rx, command_tx))
}
