use std::sync::Arc;
use anyhow::{Error, Result};

use crate::State;

pub async fn run_command(vec: Vec<u8>, state: &Arc<State>) -> Result<()> {
    match vec[0] {
        0 => {
            println!("State: {:?}", state);
            state.set_target_velocity((vec[1], vec[2])).await?;
            println!("State: {:?}", state);
            Ok(())
        }
        1 => {
            state.fire().await?;
            Ok(())
        }
        _ => {Err(Error::msg("Unknown command"))}
    }
}