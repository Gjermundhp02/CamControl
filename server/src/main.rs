use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};

mod soc_handler;
use soc_handler::handle_connection;

struct State {
    position: Arc<Mutex<[u16; 2]>>, // Steps 
    velocity: Arc<Mutex<[i16; 2]>>, // Steps per second
    target_velocity: Arc<Mutex<[i16; 2]>>, // Steps per second
    fire: Arc<Mutex<bool>>, // TODO: Timeout fire if no new command comes within 1 second
}

struct Config {
    steps_per_degree: u16,
    acceleration: u8,
    max_velocity: i16,
}

const CONFIG: Config = Config {
    steps_per_degree: 100,
    acceleration: 100,
    max_velocity: 200, // Steps per second
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8082".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    let state = Arc::new(State { 
        position: Arc::new(Mutex::new([0, 0])), 
        velocity: Arc::new(Mutex::new([0, 0])), 
        target_velocity: Arc::new(Mutex::new([0, 0])),
        fire: Arc::new(Mutex::new(false)) 
    });

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, Arc::clone(&state)));
    }

    Ok(())
}

