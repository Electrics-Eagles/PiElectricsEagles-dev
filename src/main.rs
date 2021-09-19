#[allow(non_camel_case_types)]
use loggics::main_loop;


mod clk_driver;
mod config_parse;
mod controller;
mod filter;
mod ibus;

mod logger;
mod loggics;
mod utils;
mod imu;
mod lis3dh_driver;
mod l3dgh20;


fn main() {
    main_loop();
}
