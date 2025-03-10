use std::{collections::HashMap, sync::{Arc, Mutex}};
use interpolation::lerp;
use tokio::{sync::oneshot, time::{sleep, Duration}};

use anyhow::{Ok, Result};

use crate::State;

use crate::CONFIG;

type CommandHandler = dyn Fn(Vec<u8>, &Arc<State>) -> Result<()>;

type Commands = HashMap<u8, Box<CommandHandler>>;

fn set_timeout<F>(duration: Duration, callback: F) -> impl FnOnce() -> tokio::task::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();
    
    // Return a function that cancels the timeout when called
    move || {
        tokio::spawn(async move {
            tokio::select! {
                _ = sleep(duration) => {
                    // Timeout expired without cancellation
                    callback();
                },
                _ = cancel_rx => {
                    // Timeout was canceled before execution
                    println!("Timeout canceled!");
                },
            }
        })
    }
}

fn set_target_velocity(data: Vec<u8>, state: &Arc<State>) -> Result<()> {
    let mut target_velocity = state.target_velocity.lock().unwrap();
    target_velocity[0] = lerp(&-CONFIG.max_velocity, &CONFIG.max_velocity, &(data[1] as f32 / 255.0)) as i16;
    target_velocity[1] = lerp(&-CONFIG.max_velocity, &CONFIG.max_velocity, &(data[2] as f32 / 255.0)) as i16;
    Ok(())
}

fn fire(_data: Vec<u8>, state: &Arc<State>) -> impl Fn(Vec<u8>, &Arc<State>) -> Result<()> {
    let timeout = tokio::time::timeout(Duration::from_secs(60), async {
        sleep(Duration::from_secs(1)).await;
        let mut fire = state.fire.lock().unwrap();
        *fire = false;
    });
    
    return |_data: Vec<u8>, state: &Arc<State>| -> Result<()> {
        let _ = set_timeout(Duration::from_secs(60), || {
            let mut fire = state.fire.lock().unwrap();
            *fire = false;
        });
        let mut fire = state.fire.lock().unwrap();
        *fire = true;
        Ok(())
    };
}

pub fn get_commands() -> Commands {
    let mut map:Commands = HashMap::new();
    map.insert(0 as u8, Box::new(set_target_velocity));
    return map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_state() -> Arc<State> {
        Arc::new(State { 
            position: Arc::new(Mutex::new([0, 0])), 
            velocity: Arc::new(Mutex::new([0, 0])), 
            target_velocity: Arc::new(Mutex::new([0, 0])),
            fire: Arc::new(Mutex::new(false)) 
        })
    }

    #[test]
    fn test_set_speed() -> Result<()> {
        let state = new_state();

        set_target_velocity(vec![0, 0, 255], &state)?;
        let speed = state.velocity.lock().unwrap();
        assert_eq!(speed[0], -200);
        assert_eq!(speed[1], CONFIG.max_velocity);
        Ok(())
    }
}