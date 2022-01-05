#[allow(non_camel_case_types)]
use loggics::main_loop;



mod config_parse;
mod controller;
mod filter;
mod reciver;
mod logger;
mod utils;
mod imu;
mod loggics;
mod batterycontroller;

fn main() {
    main_loop();
}
