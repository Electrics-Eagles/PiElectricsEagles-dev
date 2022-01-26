#[allow(non_camel_case_types)]
use loggics::main_loop;
use testing_hardware::test;
use test_imu::test_imu;
mod batterycontroller;
mod config_parse;
mod controller;
mod filter;
mod imu;
mod logger;
mod loggics;
mod reciver;
mod testing_hardware;
mod utils;
mod test_imu;


fn main() {
    let  mut args = std::env::args();
    if args.len() > 0 {
        let mode = args.nth(1).unwrap();
        if mode == "--testing_hardware" { test(); }
        if mode == "--test_imu" {test_imu();}
        if mode == "--normal" { main_loop(); }
        else {
            panic!("I can`t understand you");
        }
    } else {
        main_loop();
    }
}
