#![allow(unused_imports)]
use std::io::{prelude::*, Cursor};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tungstenite::accept;
use tungstenite::handshake::HandshakeError;
use tungstenite::protocol::Message;
use worders::game::GameState;
use worders::packets::{PacketFrom, PacketTo, PlayerState};
use worders::thread_pool::ThreadPool;

fn main() {
    // The number of games that can run concurrently
    let _pool = ThreadPool::new(10);
    let mut game = GameState::new(0);
    let server = TcpListener::bind("192.168.0.14:8080").unwrap();
    for stream in server.incoming() {
        thread::spawn(move || match stream {
            Ok(stream) => match handle_websocket(stream) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error Occured: {}", e);
                }
            },
            Err(e) => {
                println!("Error Occured: {}", e);
            }
        });
    }
}

fn handle_websocket(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    match accept(stream) {
        Ok(mut ws) => {
            loop {
                match ws.read_message() {
                    Ok(msg) => match msg {
                        Message::Binary(bytes) => {
                            println!("{:02x?}", bytes.as_slice());
                            let mut cursor = Cursor::new(bytes.as_slice());
                            let state = PlayerState::decode(&mut cursor);
                            println!("Player State: {:?}", state);
                            let mut send_buffer = vec![];
                            state.encode(&mut send_buffer);
                            println!("Player State Buffer: {:02x?}", send_buffer);
                            ws.write_message(Message::Binary(send_buffer)).unwrap();
                        }
                        _ => {
                            println!("Message wasn't Binary: {:?}", msg);
                        }
                    },
                    Err(e) => {
                        use tungstenite::error::Error as TError;
                        match e {
                            TError::ConnectionClosed | TError::AlreadyClosed => {
                                println!("User Disconnected!");
                                break;
                            }
                            _ => {}
                        }
                        return Err(Box::new(e));
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}
