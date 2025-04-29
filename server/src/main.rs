use interpolation::lerp;
use tokio::{net::TcpListener, time::{sleep, sleep_until, Instant, Duration}};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, pin::Pin, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use tokio::sync::Mutex;

mod soc_handler;
use soc_handler::handle_connection;

struct State {
    position: Arc<Mutex<[u16; 2]>>, // Steps 
    velocity: Arc<Mutex<[i16; 2]>>, // Steps per second
    target_velocity: Arc<Mutex<[i16; 2]>>, // Steps per second
    fire: Arc<AtomicBool>, // TODO: Timeout fire if no new command comes within 1 second
    fire_timeout: Arc<Mutex<tokio::time::Instant>>, // TODO: Timeout fire if no new command comes within 1 second
}

impl State {
    pub async fn fire(&self){
        if self.fire.load(Ordering::Relaxed){
            let mut timeout = self.fire_timeout.lock().await;
            *timeout = Instant::now()+Duration::from_secs(1);
        }
        else {
            self.fire.store(true, Ordering::Relaxed);
            let timeout_clone = Arc::clone(&self.fire_timeout);
            let fire_clone = Arc::clone(&self.fire);
            tokio::spawn(async move {
                let mut timeout = timeout_clone.lock().await;
                *timeout = Instant::now()+Duration::from_secs(1);
                let target = *timeout;
                sleep_until(target).await;
                if Instant::now() >= target {
                    fire_clone.store(false, Ordering::Relaxed);
                }
            }); 
        }
    }
    pub async fn set_target_velocity(&self, data: Vec<u8>) -> Result<()> {
        let mut target_velocity = self.target_velocity.lock().await;
        target_velocity[0] = lerp(&-CONFIG.max_velocity, &CONFIG.max_velocity, &(data[1] as f32 / 255.0)) as i16;
        target_velocity[1] = lerp(&-CONFIG.max_velocity, &CONFIG.max_velocity, &(data[2] as f32 / 255.0)) as i16;
        Ok(())
    }
}

type Command = FnMut(Vec<u8>, &Arc<State>) -> impl std::future::Future<Output = Result<()>>;

struct Config {
    steps_per_degree: u16,
    acceleration: u8,
    max_velocity: i16,
    commands: HashMap<u8, Command>,
}



impl Config {
    pub fn new() -> Self {
        let mut commands: HashMap<u8, Command> = HashMap::new();
        commands.insert(0, async |data: Vec<u8>, state: &Arc<State>| {
            state.set_target_velocity(data).await?;
        });
        let func = async |data: Vec<u8>, state: &Arc<State>| {
            state.set_target_velocity(data).await?;
        };
        Self { steps_per_degree: 100, acceleration: 100, max_velocity: 200, commands }
    }
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
        fire: Arc::new(AtomicBool::new(false)), 
        fire_timeout: Arc::new(Mutex::new(tokio::time::Instant::now())), // Initialize fire_timeout
    });

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, Arc::clone(&state)));
    }

    Ok(())
}

