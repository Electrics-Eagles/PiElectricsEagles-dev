use core::time;
use std::thread;
use std::time::SystemTime;

pub fn sin(input: f64) -> f64 {
    return input.sin();
}

pub fn sqrt(input: f32) -> f32 {
    input.sqrt()
}

pub fn abs(input: f32) -> f32 {
    input.abs()
}

pub fn delay(input: u64) {
    thread::sleep(time::Duration::from_millis(input));
}
