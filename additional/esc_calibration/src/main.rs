mod config_parse;
mod controller;
use crate::config_parse::*;
use crate::controller::*;
use std::process;
use std::{thread, time};
fn main() {
    let number_motor: u16 = std::env::args()
        .nth(1)
        .expect("no pattern given")
        .parse()
        .unwrap();
    println!("Number of motor will calibrate: {}", number_motor);
    if (number_motor > 3 || number_motor < 0) {
        println!("Incorrect entering number of motor!");
        std::process::exit(0);
    }
    
    let mut controller = Controller::new();
    println!("You must ready your safety before calibration motor. Please don't touch a motor to avoid injury");
    println!("Motor will calibrate!!!");
    println!("Sending a maximum pulse");
    if (number_motor == 0) {
        controller.run_single_motor(pwm_pca9685::Channel::C0, 2000);
    } else if (number_motor == 1) {
        controller.run_single_motor(pwm_pca9685::Channel::C1, 2000);
    } else if (number_motor == 2) {
        controller.run_single_motor(pwm_pca9685::Channel::C2, 2000);
    } else if (number_motor == 3) {
        controller.run_single_motor(pwm_pca9685::Channel::C3, 2000);
    }

    println!("Turn on motor via battery and wait 2 second before entering any key.");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);

    println!("Sending a minimum pulse");
    if (number_motor == 0) {
        controller.run_single_motor(pwm_pca9685::Channel::C0, 1000);
    } else if (number_motor == 1) {
        controller.run_single_motor(pwm_pca9685::Channel::C1, 1000);
    } else if (number_motor == 2) {
        controller.run_single_motor(pwm_pca9685::Channel::C2, 1000);
    } else if (number_motor == 3) {
        controller.run_single_motor(pwm_pca9685::Channel::C3, 1000);
    }
    println!("Motor will run with pulse 1200 us in next the 10 seconds. Please don't touch a motor to avoid injury. Just wait for 10 seconds before running motor");
    std::thread::sleep(time::Duration::from_millis(10000));
    if (number_motor == 0) {
        controller.run_single_motor(pwm_pca9685::Channel::C0, 1200);
    } else if (number_motor == 1) {
        controller.run_single_motor(pwm_pca9685::Channel::C1, 1200);
    } else if (number_motor == 2) {
        controller.run_single_motor(pwm_pca9685::Channel::C2, 1200);
    } else if (number_motor == 3) {
        controller.run_single_motor(pwm_pca9685::Channel::C3, 1200);
    }

    println!("Press any key to stop to run motor");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);

    if (number_motor == 0) {
        controller.run_single_motor(pwm_pca9685::Channel::C0, 1000);
    } else if (number_motor == 1) {
        controller.run_single_motor(pwm_pca9685::Channel::C1, 1000);
    } else if (number_motor == 2) {
        controller.run_single_motor(pwm_pca9685::Channel::C2, 1000);
    } else if (number_motor == 3) {
        controller.run_single_motor(pwm_pca9685::Channel::C3, 1000);
    }
}
