use commands::run_command;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::StreamExt;
use std::sync::Arc;

use crate::State;

mod commands;

pub async fn handle_connection(stream: tokio::net::TcpStream, state: Arc<State>) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established"); // Implement a uuid write lock so that only one connection can wrtite at a time

    while let Some(msg) = ws_stream.next().await {
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