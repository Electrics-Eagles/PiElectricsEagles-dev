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
mod loggics;
mod mpu6050;
mod simple_logger;
fn version_display() {
    let mut config = config_parser::new();

    simple_logger::write_log(LevelOfLog::INFO, "VERISONS OF MODULES ".parse().unwrap());
    simple_logger::write_log(LevelOfLog::INFO, logger_verison().parse().unwrap());
    simple_logger::write_log(LevelOfLog::INFO, config.config_parser_version().parse().unwrap());
    // simple_logger::logger(1, FILE_LOG, sbus_verison().parse().unwrap());
}

fn main() {
    init_log(); // initialize for logger
    version_display(); // call function that display software verison
    main_loop();
}
