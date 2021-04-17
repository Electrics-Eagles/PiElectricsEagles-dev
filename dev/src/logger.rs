use std::fs::File;
use std::io::prelude::*;

const LOG_PATH: str = *"/var/log/pi_drone.log";

pub struct Logger {
    file_: file,
}
pub struct LoggingStruct {
    pub acc_z: f64,
    pub acc_y: f64,
    pub acc_x: f64,
    pub gyro_x: f64,
    pub gyro_y: f64,
    pub gyro_z: f64,
    pub reciver_ch6: f64,
    pub pitch_level_correction: f64,
    pub roll_level_correction: f64,
    pub angle_pitch_acc: f64,
    pub angle_roll_acc: f64,
    pub pid_roll_setpoint: f64,
    pub pid_pitch_setpoint: f64,
    pub pid_yaw_setpoint: f64,
    pub pid_output_roll: f64,
    pub pid_output_pitch: f64,
    pub esc_1: f64,
    pub esc_2: f64,
    pub esc_3: f64,
    pub esc_4: f64,
    pub time_spent: f64,
}
impl Logger {
    pub fn new() -> File {
        let mut file = File::create(LOG_PATH).unwrap();
        file.write_all("PiElectricsEagles Log".as_ref()).unwrap();
        file
    }

    pub fn write_to_log(mode: u8, log: LoggingStruct) {


    }
}
