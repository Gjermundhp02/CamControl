use tokio::net::TcpListener;
use anyhow::Result;
use std::sync::Arc;

mod soc_handler;
use soc_handler::handle_connection;

mod state;
use state::State;

mod io;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8082".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    let state = Arc::new(State::default());
    print!("State: {:?}", state);
    
    if option_env!("GPIO").is_some() {
        println!("GPIO is enabled");
        io::handle_io(Arc::clone(&state))?;
    }

    while let Ok((stream, _)) = listener.accept().await {
        // Intentionaly only allows one connection at a time
        if let Err(e) = handle_connection(stream, Arc::clone(&state)).await {
            println!("Error handling connection: {:?}", e);
        }
    }

    Ok(())
}

