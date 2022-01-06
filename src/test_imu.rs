use crate::config_parse::*;
use crate::controller::*;
use crate::filter::{ABfilter, low_pass_filter};
use crate::filter::Filter;
use crate::reciver::*;
use crate::imu::imu;
use crate::logger::*;
use crate::utils::abs;
use crate::utils::sin;
use core::time;
use std::thread;
use std::time::SystemTime;
use crate::reciver;
use crate::utils::delay;

/*
Code that allows to connect your drone to pc and test the gyroscope positon and see it is located correctly 
*/

pub fn test_imu() {

    let mut config = config_parser::new();



    let axis_assignment_gyro = config.imu_config_parser().axis_assignment_gyro;
    let axis_assignment_acc = config.imu_config_parser().axis_assignment_acc;
    let gyro_axis_reverse = config.imu_config_parser().reversed_axis_gyro;
    let acc_axis_reverse = config.imu_config_parser().reversed_axis_acc;

    let mut imu = imu::new();
    delay(5000);
    println!("Calibrate Gyro . Do not touch drone including squrrels");
    imu.calibrate();

    loop {
        let gyro_data = imu.get_normalised_gyro_data(axis_assignment_gyro.clone(),gyro_axis_reverse.clone());
        let acc_data = imu.get_acc_data(axis_assignment_acc.clone(),acc_axis_reverse.clone());


        println("{},{},{},{},{},{}",gyro_data.roll,gyro_data.pitch,gyro_data.yaw,acc_data.roll,acc_data.pitch,acc_data.yaw);
        delay(1000);

    }

}