#[macro_use]
extern crate lazy_static; // crate for convert from u8 to hex
extern crate mut_static;

use crate::config_parse::config_parser;
use loggics::main_loop;

use crate::logger::*;

mod clk_driver;
mod config_parse;
mod controller;
mod ibus;
mod logger;
mod loggics;
mod mpu6050;

fn main() {
    main_loop();
}
