use crate::perpare_esc::calibrate_esc;
use crate::perpare_esc::esc_test;
#[allow(non_camel_case_types)]
use loggics::main_loop;
use test_imu::test_imu;
use testing_hardware::test;

mod batterycontroller;
mod config_parse;
mod controller;
mod filter;
mod imu;
mod logger;
mod loggics;
mod perpare_esc;
mod reciver;
mod test_imu;
mod testing_hardware;
mod utils;

fn main() {
    let mut args = std::env::args();
    if args.len() > 0 {
        let mode = args.nth(1).unwrap();
        if mode == "--testing_hardware" {
            test();
        }
        if mode == "--test_imu" {
            test_imu();
        }
        if mode == "--normal" {
            main_loop();
        }
        if mode == "--calibrate_esc" {
            calibrate_esc();
        }
        if mode == "--esc_test" {
            esc_test();
        } else {
            panic!("I can`t understand you");
        }
    } else {
        main_loop();
    }
}
