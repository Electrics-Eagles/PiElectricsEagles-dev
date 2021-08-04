
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



fn main() {
    main_loop();
}
