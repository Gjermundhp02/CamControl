use commands::run_command;
use futures_util::stream::SplitSink;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::{Bytes, Error as WsError};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::{anyhow, Result};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;

use crate::State;

mod commands;

fn update_client(write: SplitSink<WebSocketStream<TcpStream>, Message>, state: Arc<State>) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        let mut write = write;
        loop {
            let pos = state.position.lock().await;
            let fire = state.fire_timeout.lock().await;
            let data = vec![
                0, // Type of message
                (pos.0 / *state.config.steps_per_degree) as u8, // Angle-x
                (pos.1 / *state.config.steps_per_degree) as u8, // Angle-y
                fire.is_some() as u8, // Fire
            ];
            let dat = data.clone();
            if let Err(err) = write.send(Message::Binary(Bytes::from(data))).await {
                match err {
                    WsError::AlreadyClosed => {
                        println!("Connection closed");
                        break;
                    }
                    _ => {
                        println!("Error sending message: {:?}", err);
                        return Err(anyhow!("Error sending message: {:?}", err));
                    }
                }                
            }
            else {
                println!("Sent message: {:?}", dat);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        Ok(())
    })
}

pub async fn handle_connection(stream: TcpStream, state: Arc<State>) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established"); // Implement a uuid write lock so that only one connection can wrtite at a time
    let (write, mut read) = ws_stream.split();
    update_client(write, state.clone());

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Binary(data) => {
                let dat = data.to_vec();
                println!("Received binary data: {:?}", dat);
                run_command(dat, &state).await?;
            }
            _ => {}
        }
    }

    Ok(())
}