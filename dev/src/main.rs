#[macro_use]
extern crate lazy_static; // crate for convert from u8 to hex
extern crate mut_static;

use crate::config_parse::config_parser;
use loggics::main_loop;

use crate::simple_logger::*;

mod clk_driver;
mod config_parse;
mod controller;
mod ibus;
mod logger;
mod loggics;
mod mpu6050;
fn version_display() {
    let mut config = config_parser::new();
}

fn main() {
    version_display(); // call function that display software verison
    main_loop();
}
