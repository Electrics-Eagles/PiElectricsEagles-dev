use linux_embedded_hal::{Delay, I2cdev};
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{
    thread,
    time::{self, SystemTime},
};

use crate::config_parse::*;
use crate::controller::*;

mod config_parse;
mod controller;
//mod simple_logger;
fn main() {
    println!("We test ESC all channels. Please make sure your workspace is clear before starting to calibrate ESC. Avoid any injuries.");
    let mut controller = Controller::new();
    println!("Initialized and created object of ESC motor driver");

    println!("ESC motor started to calibrate");
    controller.test_esc(2000);

    thread::sleep(time::Duration::from_millis(500));
    println!("Please connect battery to ESC controller.");
    thread::sleep(time::Duration::from_millis(3000));

    controller.test_esc(1000);
    println!("ESC motor finished to calibrate");
    thread::sleep(time::Duration::from_millis(6000));

    println!("ESC motor started to check. Pulse width = 1100 us.");
    println!("Please take care that motor will start to rotate. Don't touch motor now to avoid any injuries!");
    println!("This test will take about five seconds...");
    controller.test_esc(1100);
    thread::sleep(time::Duration::from_millis(5000));
    println!("Motor turn off now...");
    println!("Please disconnect battery from ESC motor for safety precaution...");
}
