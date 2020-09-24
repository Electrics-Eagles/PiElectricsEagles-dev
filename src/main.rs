
const FILE_LOG:bool=false;

use linux_embedded_hal::I2cdev;
use crate::config_parse::{esc_config_parser, get_pids, config_parser_version};
use crate::mpu6050::driver_mpu6050_version;
use crate::controller::get_esc_verison;
use crate::sbus::sbus_verison;
use crate::simple_logger::logger_verison;

mod simple_logger;
mod config_parse;
mod sbus;
mod controller;
mod mpu6050;



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
    let pids_values = get_pids(); // get pid config
    println!("{}",pids_values.get(0).unwrap()[0]);
    simple_logger::logger(1, FILE_LOG, "CONFIG READ&PARSE=OK".parse().unwrap());
    let motors_config = esc_config_parser();
    println!("{}",motors_config.amount);
    let i2c_controller = controller::external_pwm_prepare(motors_config.port, motors_config.amount, motors_config.driver);
    let sbus=config_parse::sbus_receiver_conifg();
    sbus::read_sbus(sbus.baudrate, sbus.parity, sbus.data_bits as u8, sbus.stop_bit as u8, sbus.port).unwrap();
    mpu6050::mpu6050_perpare().get_gyro().unwrap().x;


}
