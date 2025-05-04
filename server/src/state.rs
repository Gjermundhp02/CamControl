use tokio::time::{Instant, Duration, sleep_until};
use anyhow::Result;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::sync::Mutex;

use crate::io;

#[derive(Debug)]
pub struct Config {
    pub steps_per_degree: Arc<u16>,
    pub acceleration: Arc<u8>,
    pub max_velocity: Arc<i8>,
}

impl Config {
    fn default() -> Self {
        Self { 
            steps_per_degree: Arc::new(100), 
            acceleration: Arc::new(3), 
            max_velocity: Arc::new(127)
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub position: Arc<Mutex<(i16, i16)>>, // Steps 
    pub velocity: Arc<Mutex<(i8, i8)>>, // Steps per second
    pub target_velocity: Arc<Mutex<(i8, i8)>>, // Steps per second
    pub fire: Arc<AtomicBool>, // TODO: Timeout fire if no new command comes within 1 second
    pub fire_timeout: Arc<Mutex<tokio::time::Instant>>,
    pub config: Config,
}

impl State {
    pub fn default() -> Self {
        Self { 
            position: Default::default(),
            velocity: Default::default(),
            target_velocity: Default::default(),
            fire: Default::default(),
            fire_timeout: Arc::new(Mutex::new(tokio::time::Instant::now())), // Initialize fire_timeout
            config: Config::default(),
        }
    }

    pub async fn fire(&self) -> Result<()> {
        const FIRE_TIMEOUT: u64 = 500;
        if self.fire.load(Ordering::Relaxed){
            // If fire is already set, just update the timeout
            let mut timeout = self.fire_timeout.lock().await;
            *timeout = Instant::now()+Duration::from_millis(FIRE_TIMEOUT);
            Ok(())
        }
        else {
            // If fire is not set, set it and start a timeout
            if option_env!("GPIO").is_some() {
                io::set_fire()?;
            }
            self.fire.store(true, Ordering::Relaxed);
            let timeout_clone = Arc::clone(&self.fire_timeout);
            let fire_clone = Arc::clone(&self.fire);
            tokio::spawn(async move {
                let mut timeout = timeout_clone.lock().await;
                *timeout = Instant::now()+Duration::from_millis(FIRE_TIMEOUT);
                let target = *timeout;
                // Wait until the timeout is reached
                sleep_until(target).await;
                while Instant::now() < target {
                    sleep_until(target).await;                    
                }
                fire_clone.store(false, Ordering::Relaxed);
                if option_env!("GPIO").is_some() {
                    io::reset_fire();
                }
            });
            Ok(())
        }
    }

    /*
     * @param data: (u8, u8) - The first byte is the x-dir (0-254), the second byte is the y-dir (0-254)
     */
    pub async fn set_target_velocity(&self, data: (u8, u8)) -> Result<()> {
        let mut target_velocity = self.target_velocity.lock().await;
        // The input data is treated as a percentage of the max velocity
        target_velocity.0 = (data.0-255/2) as i8;
        target_velocity.1 = (data.1-255/2) as i8;
        Ok(())
    }
}