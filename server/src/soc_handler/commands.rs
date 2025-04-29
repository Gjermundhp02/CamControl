use std::{collections::HashMap, ops::DerefMut, sync::{atomic::Ordering, Arc}};
use interpolation::lerp;
use tokio::{sync::oneshot, time::{sleep, Duration}, sync::Mutex};

use anyhow::{Ok, Result};

use crate::State;

use crate::CONFIG;

type CommandHandler = dyn Fn(Vec<u8>, &Arc<State>) -> Result<()>;

type Commands = HashMap<u8, Box<CommandHandler>>;

async fn set_target_velocity(data: Vec<u8>, state: &Arc<State>) -> Result<()> {
    let mut target_velocity = state.target_velocity.lock().await;
    target_velocity[0] = lerp(&-CONFIG.max_velocity, &CONFIG.max_velocity, &(data[1] as f32 / 255.0)) as i16;
    target_velocity[1] = lerp(&-CONFIG.max_velocity, &CONFIG.max_velocity, &(data[2] as f32 / 255.0)) as i16;
    Ok(())
}

pub fn get_commands() -> Commands {
    let mut map:Commands = HashMap::new();
    map.insert(0 as u8, Box::new(set_target_velocity));
    return map
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;

    use super::*;

    fn new_state() -> Arc<State> {
        Arc::new(State { 
            position: Arc::new(Mutex::new([0, 0])), 
            velocity: Arc::new(Mutex::new([0, 0])), 
            target_velocity: Arc::new(Mutex::new([0, 0])),
            fire: Arc::new(AtomicBool::new(false)),
            fire_timeout: Arc::new(Mutex::new(tokio::time::Instant::now()))
        })
    }

    #[tokio::test]
    async fn test_set_speed() -> Result<()> {
        let state = new_state();

        set_target_velocity(vec![0, 0, 255], &state)?;
        let speed = state.velocity.lock().await;
        assert_eq!(speed[0], -200);
        assert_eq!(speed[1], CONFIG.max_velocity);
        Ok(())
    }
}