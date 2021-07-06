#[macro_use]
extern crate lazy_static; // crate for convert from u8 to hex
extern crate mut_static;

use crate::config_parse::config_parser;
use loggics::main_loop;

use crate::logger::*;

mod clk_driver;
mod config_parse;
mod controller;
mod filter;
mod ibus;
mod l3dgh20;
mod logger;
mod loggics;
mod untils;
mod lis3dh_driver;


fn main() {
    main_loop();
}
