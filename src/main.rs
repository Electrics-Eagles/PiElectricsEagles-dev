
const FILE_LOG:bool=false;


use loggics::main_loop;

use crate::config_parse::{esc_config_parser, get_pids, config_parser_version, mpu_config_parser};
use crate::mpu6050::driver_mpu6050_version;
use crate::controller::get_esc_verison;
use crate::sbus::sbus_verison;
use crate::simple_logger::logger_verison;

mod simple_logger;
mod config_parse;
mod sbus;
mod controller;
mod mpu6050;
mod loggics;


fn version_display(){
    simple_logger::logger(1, FILE_LOG, "VERISONS OF MODULES ".parse().unwrap());
    simple_logger::logger(1, FILE_LOG, logger_verison().parse().unwrap());
    simple_logger::logger(1, FILE_LOG, config_parser_version().parse().unwrap());
    simple_logger::logger(1, FILE_LOG,sbus_verison().parse().unwrap());
    simple_logger::logger(1, FILE_LOG, get_esc_verison().parse().unwrap());
    simple_logger::logger(1, FILE_LOG, driver_mpu6050_version().parse().unwrap());
}

fn main() {
    version_display(); // call function that display software verison
   main_loop();




}
