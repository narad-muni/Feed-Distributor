use std::{
    io::{Read, Write},
    net::Shutdown,
    sync::Arc,
};

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use tungstenite::{accept, handshake::server::NoCallback, HandshakeError, ServerHandshake, WebSocket};

use crate::{
    constants::{EVENT_CAPACITY, TCP_LISTENER_TOKEN, WS_LISTENER_TOKEN},
    globals::CLIENTS_LIST,
    types::{
        client_profile::{ClientProfile, Connection},
        packet::InputPacket,
        settings,
    },
    utils::error_utils::{interrupted, would_block},
};

pub struct ClientInput {
    listeners: [TcpListener; 2],
    poll: Poll,
}

impl ClientInput {
    pub fn new() -> Self {
        let tcp_address = settings::get().tcp_address.clone();
        let ws_address = settings::get().ws_address.clone();

        let mut tcp_listener = TcpListener::bind(tcp_address.parse().unwrap()).unwrap();
        let mut ws_listener = TcpListener::bind(ws_address.parse().unwrap()).unwrap();

        let poll = Poll::new().unwrap();

        poll.registry()
            .register(&mut tcp_listener, TCP_LISTENER_TOKEN, Interest::READABLE)
            .unwrap();

        poll.registry()
            .register(&mut ws_listener, WS_LISTENER_TOKEN, Interest::READABLE)
            .unwrap();

        Self {
            listeners: [tcp_listener, ws_listener],
            poll,
        }
    }

    pub fn start_input(&mut self) {
        loop {
            let mut events = Events::with_capacity(EVENT_CAPACITY);

            // Load all events
            if let Err(err) = self.poll.poll(&mut events, None) {
                if interrupted(&err) {
                    continue;
                }
                return;
            }

            // Process each event
            for event in events.iter() {
                match event.token() {
                    // For new connection events
                    event_token @ (TCP_LISTENER_TOKEN | WS_LISTENER_TOKEN) => loop {
                        // Continuosly accept new connections
                        match self.listeners[event_token.0].accept() {
                            // Handle accepted connection
                            Ok((stream, _)) => handle_connection(stream, event_token, &self.poll),
                            // Wait for more connections
                            Err(e) if interrupted(&e) => continue,
                            // No more connections
                            _ => break,
                        }
                    },
                    token => {
                        // For other events
                        handle_request(token.0);
                    }
                }
            }
        }
    }
}

fn handshake_ws(
    stream: TcpStream,
) -> Result<WebSocket<TcpStream>, HandshakeError<ServerHandshake<TcpStream, NoCallback>>> {
    match accept(stream) {
        Ok(ws) => Ok(ws),
        Err(HandshakeError::Interrupted(mut mid)) => loop {
            // Interrupted while doing handshake
            // Try handshake till error or success
            match mid.handshake() {
                Ok(ws) => break Ok(ws),
                Err(HandshakeError::Interrupted(m)) => {
                    mid = m;
                    continue;
                }
                Err(e) => return Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

pub fn handle_connection(mut stream: TcpStream, event_token: Token, poll: &Poll) {
    // Reserve index
    let idx = CLIENTS_LIST.reserve();

    // Register stream with idx as identifier
    poll.registry()
        .register(&mut stream, Token(idx), Interest::READABLE)
        .unwrap();

    // Create connection
    let conn = match event_token {
        TCP_LISTENER_TOKEN => Connection::Tcp(stream),
        WS_LISTENER_TOKEN => match handshake_ws(stream) {
            Ok(conn) => Connection::Ws(conn),
            Err(_) => {
                handle_disconnection(idx);
                return;
            }
        },
        _ => unreachable!(),
    };

    println!("Connected");
    // Create client profile and insert it
    CLIENTS_LIST.insert_at(ClientProfile::create_empty(Arc::new(conn)), idx);
}

pub fn handle_request(idx: usize) {
    let mut packet = InputPacket::new();

    let client_profile = CLIENTS_LIST.get_mut(idx).as_mut().unwrap();

    if let Connection::Tcp(stream) = &mut client_profile.conn {
        loop {
            match stream.read(&mut packet.0) {
                Ok(0) => {
                    // Connection closed
                    handle_disconnection(idx);
                    return;
                }
                Ok(size) => {
                    // Read data
                    packet.1 += size;
                    continue;
                }
                Err(e) if interrupted(&e) => {
                    // Waiting for more data to read from socket
                    continue;
                }
                Err(e) if would_block(&e) => {
                    // No more data available
                    break;
                }
                Err(_) => {
                    // Something else went wrong
                    // Disconnect
                    handle_disconnection(idx);
                    return;
                }
            };
        }

        // println!("Received {:?}", String::from_utf8_lossy(&buffer[..sz]));

        stream.write_all(b"Hello").unwrap();
        stream.flush().unwrap();
    } else if let Connection::Ws(ws) = &mut client_profile.conn {
        let msg = ws.read().unwrap();

        ws.send(msg);
        // println!("Received {:?}", msg);
    }
}

pub fn handle_init() {}

pub fn handle_token_subscribe() {}

pub fn handle_token_unsubscribe() {}

pub fn handle_udp_switch() {}

pub fn handle_invalid_request() {}

pub fn handle_disconnection(idx: usize) {
    let client_profile = CLIENTS_LIST.remove(idx).unwrap();

    if let Connection::Ws(mut ws) = client_profile.conn {
        let _ = ws.close(None);
    } else if let Connection::Tcp(stream) = client_profile.conn {
        let _ = stream.shutdown(Shutdown::Both);
    }

    println!("{} disconnected", idx);
}
