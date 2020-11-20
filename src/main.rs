
#[macro_use]
extern crate lazy_static;// crate for convert from u8 to hex
extern crate mut_static;
const FILE_LOG: bool = false;

use loggics::main_loop;

use crate::config_parse::config_parser_version;

use crate::simple_logger::logger_verison;

mod config_parse;
mod controller;
mod loggics;
mod mpu6050;
mod ibus;
mod simple_logger;

fn version_display() {
    simple_logger::logger(1, FILE_LOG, "VERISONS OF MODULES ".parse().unwrap());
    simple_logger::logger(1, FILE_LOG, logger_verison().parse().unwrap());
    simple_logger::logger(1, FILE_LOG, config_parser_version().parse().unwrap());
   // simple_logger::logger(1, FILE_LOG, sbus_verison().parse().unwrap());
   
}

fn main() {
    version_display(); // call function that display software verison
    main_loop();
}
