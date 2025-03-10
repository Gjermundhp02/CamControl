use tokio_tungstenite::{accept_async, tungstenite::protocol::frame::coding::Data};
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::State;

mod commands;

pub async fn handle_connection(stream: tokio::net::TcpStream, state: Arc<State>) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established"); // Implement a uuid write lock so that only one connection can wrtite at a time

    while let Some(msg) = ws_stream.next().await {
        match msg? {
            Message::Binary(data) => {
                println!("Received binary data: {:?}", data.to_vec());
            }
            _ => {}
        }
    }

    Ok(())
}