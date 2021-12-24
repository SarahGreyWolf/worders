#![allow(unused_imports)]
use std::io::{prelude::*, Cursor};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::accept;
use tungstenite::handshake::HandshakeError;
use tungstenite::protocol::Message;
use worders::game::GameState;
use worders::packets::*;
use worders::thread_pool::ThreadPool;

fn main() {
    // The number of games that can run concurrently
    let _pool = ThreadPool::new(10);
    let mut game = Arc::new(Mutex::new(GameState::new(0)));
    let server = TcpListener::bind("192.168.0.14:8080").unwrap();
    let game_clone = game.clone();
    for stream in server.incoming() {
        thread::spawn(move || match stream {
            Ok(stream) => match handle_websocket(stream, game_clone) {
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

fn handle_websocket(
    stream: TcpStream,
    game: Arc<Mutex<GameState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match accept(stream) {
        Ok(mut ws) => {
            let active_player = 0;
            loop {
                match ws.read_message() {
                    Ok(msg) => match msg {
                        Message::Binary(bytes) => {
                            let mut cursor = Cursor::new(bytes.as_slice());
                            let packet = Packets::decode(&mut cursor);
                            match packet {
                                Packets::Ack(ack) => println!("{:?}", ack),
                                Packets::PlayerState(state) => println!("{:?}", state),
                                Packets::Place(place) => println!("{:?}", place),
                                Packets::GameState(state) => println!("{:?}", state),
                                Packets::Unknown => eprint!("Unknown Packet Received"),
                            }
                            //ws.write_message(Message::Binary(send_buffer)).unwrap();
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
