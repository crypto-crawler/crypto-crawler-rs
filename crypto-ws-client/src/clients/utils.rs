use http::Uri;
use log::*;
use native_tls::{HandshakeError as TlsHandshakeError, TlsConnector};
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    thread,
    time::{self, Duration},
};
use tungstenite::{
    client::{self, AutoStream, IntoClientRequest},
    error::{TlsError, UrlError},
    handshake::{client::Response, HandshakeError},
    stream::{Mode, NoDelay, Stream as StreamSwitcher},
    Error, Result, WebSocket,
};

fn wrap_stream(stream: TcpStream, domain: &str, mode: Mode) -> Result<AutoStream> {
    match mode {
        Mode::Plain => Ok(StreamSwitcher::Plain(stream)),
        Mode::Tls => {
            let connector = TlsConnector::builder().build().map_err(TlsError::Native)?;
            connector
                .connect(domain, stream)
                .map_err(|e| match e {
                    TlsHandshakeError::Failure(f) => TlsError::Native(f).into(),
                    TlsHandshakeError::WouldBlock(_) => {
                        panic!("Bug: TLS handshake not blocked")
                    }
                })
                .map(StreamSwitcher::Tls)
        }
    }
}

// copied from https://github.com/snapview/tungstenite-rs/blob/master/src/client.rs#L167
fn connect_to_some(
    addrs: &[SocketAddr],
    uri: &Uri,
    mode: Mode,
    timeout: Option<u64>,
) -> Result<AutoStream> {
    let domain = uri.host().ok_or(Error::Url(UrlError::NoHostName))?;
    for addr in addrs {
        debug!("Trying to contact {} at {}...", uri, addr);
        if let Ok(raw_stream) = TcpStream::connect(addr) {
            if let Some(seconds) = timeout {
                let _ = raw_stream.set_read_timeout(Some(Duration::from_secs(seconds)));
            }
            if let Ok(stream) = wrap_stream(raw_stream, domain, mode) {
                return Ok(stream);
            }
        }
    }
    Err(Error::Url(UrlError::UnableToConnect(uri.to_string())))
}

// Usually ws_stream.read_message() blocks forever,
// with this function, it returns after 5 seconds if no data comming in
fn connect_with_timeout(
    url: &str,
    timeout: Option<u64>,
) -> Result<(WebSocket<AutoStream>, Response)> {
    let request = url.into_client_request()?;

    let uri = request.uri();
    let mode = client::uri_mode(uri)?;
    let host = request
        .uri()
        .host()
        .ok_or(Error::Url(UrlError::NoHostName))?;
    let port = uri.port_u16().unwrap_or(match mode {
        Mode::Plain => 80,
        Mode::Tls => 443,
    });
    let addrs = (host, port).to_socket_addrs()?;
    let mut stream = connect_to_some(addrs.as_slice(), &request.uri(), mode, timeout)?;
    NoDelay::set_nodelay(&mut stream, true)?;

    client::client(request, stream).map_err(|e| match e {
        HandshakeError::Failure(f) => f,
        HandshakeError::Interrupted(_) => panic!("Bug: blocking handshake not blocked"),
    })
}

// This function is equivalent to tungstenite::connect(), with an additional benefit that
// it can make read_message() timeout after 5 seconds
pub(super) fn connect_with_retry(url: &str, timeout: Option<u64>) -> WebSocket<AutoStream> {
    let mut res = connect_with_timeout(url, timeout);
    let mut count: i8 = 1;
    while res.is_err() && count < 3 {
        warn!(
            "Error connecting to {}, {}, re-connecting now...",
            url,
            res.unwrap_err()
        );
        thread::sleep(time::Duration::from_secs(3));
        res = tungstenite::connect(url);
        count += 1;
    }

    match res {
        Ok((ws_stream, _)) => ws_stream,
        Err(err) => {
            error!("Error connecting to {}, {}, aborted", url, err);
            panic!("Error connecting to {}, {}, aborted", url, err);
        }
    }
}

pub(super) const CHANNEL_PAIR_DELIMITER: char = ':';
