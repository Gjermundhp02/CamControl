use std::{sync::Arc, time::Duration};
use anyhow::Result;
use tokio::{sync::Mutex, time::sleep};

use crate::State;

type ArcTouple<T> = Arc<Mutex<(T, T)>>;

pub fn handle_io(state: Arc<State>) -> Result<()> {
    handle_step(
        Arc::clone(&state.position),
        Arc::clone(&state.velocity),
        Arc::clone(&state.target_velocity),
        Arc::clone(&state.config.acceleration),
    )?;
    Ok(())
}

/*
 * Handles gpio interactions for the stepper motors
 */
fn handle_step(pos: ArcTouple<i16>, vel: ArcTouple<i8>, target_vel: ArcTouple<i8>, acc: Arc<u8>) -> Result<()>{
    // Set up gpio
    use rppal::gpio::Gpio;
    let gpio = Gpio::new()?;
    let mut step_pin_x = gpio.get(17)?.into_output();
    let mut step_pin_y = gpio.get(18)?.into_output();
    let mut dir_pin_x = gpio.get(19)?.into_output();
    let mut dir_pin_y = gpio.get(19)?.into_output();
    

    tokio::spawn(async move {
        loop {
            let mut pos = pos.lock().await;
            let mut vel = vel.lock().await;
            let target_vel = target_vel.lock().await;

            // Todo: Make shure that -acc does not owershoot the target velocity
            // Todo: Add brake pin 
            // Accelerate
            if vel.0 < target_vel.0 {
                vel.0 += *acc as i8;
            } else if vel.0 > target_vel.0 {
                vel.0 -= *acc as i8;
            }
            // Set the direction of the step
            if vel.0 > 0 {
                pos.0 += 1;
                dir_pin_x.set_high();
                step_pin_x.set_high();
            }
            else {
                pos.0 -= 1;
                dir_pin_x.set_low();
                step_pin_x.set_high();
            }

            // Accelerate
            if vel.1 < target_vel.1 {
                vel.1 += *acc as i8;
                
            } else if vel.1 > target_vel.1 {
                vel.1 -= *acc as i8;
            }
            // Set the direction of the step
            if vel.1 > 0 {
                pos.1 += 1;
                dir_pin_y.set_high();
                step_pin_y.set_high();
            }
            else {
                pos.1 -= 1;
                dir_pin_y.set_low();
                step_pin_y.set_high();
            }

            // Set the pin high for one ms and sleep the remaining duration
            // This does not account for the time that the above code takes to run
            let duration = Duration::from_millis(60*1000/255-1);
            sleep(Duration::from_millis(1)).await;
            step_pin_x.set_low();
            sleep(duration).await;
        }
    });
    Ok(())
}

/*
 * Sets the fire pin to high
 */
pub fn set_fire() -> Result<()> {
    use rppal::gpio::Gpio;
    let gpio = Gpio::new()?;
    let mut fire_pin = gpio.get(20)?.into_output();
    fire_pin.set_high();
    Ok(())
}

/*
 * Sets the fire pin to low
 * Cannot be allowed to fail
 */
pub fn reset_fire() {
    use rppal::gpio::Gpio;
    let mut gpio = Gpio::new();
    while gpio.is_err() {
        println!("Error: {:?}", gpio);
        gpio = Gpio::new();
    }
    if let Ok(gpio) = gpio {
        let mut fire_pin = gpio.get(20);
        while fire_pin.is_err() {
            println!("Error: {:?}", fire_pin);
            fire_pin = gpio.get(20);
        }
        if let Ok(pin) = fire_pin {
            let mut pin2 = pin.into_output();
            pin2.set_low();
        } else {
            println!("Error: {:?}", fire_pin);
            
        }
    }
}