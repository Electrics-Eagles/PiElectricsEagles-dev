use linux_embedded_hal::{Delay, I2cdev};
extern crate pid;
use crate::clk_driver::ClkDriver;
use crate::config_parse::*;
use crate::controller::*;
use crate::filter::*;
use crate::ibus::*;
use crate::logger::*;
use pid::Pid;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{
    thread,
    time::{self, SystemTime},
};
use crate::lis3dh_driver::LIS3DH_Driver;

use crate::l3dgh20::L3GD20H_Driver;
use crate::untils::sin;
use cgmath::Zero;
use std::borrow::Borrow;
use cgmath::num_traits::abs;
use ang::asin;

static mut pid_error_temp: f64 = 0.0;

static mut pid_i_mem_roll: f64 = 0.0;
static mut pid_i_mem_pitch: f64 = 0.0;
static mut pid_i_mem_yaw: f64 = 0.0;

static mut pid_roll_setpoint: f64 = 0.0;
static mut pid_pitch_setpoint: f64 = 0.0;
static mut pid_yaw_setpoint: f64 = 0.0;

static mut pid_output_roll: f64 = 0.0;
static mut pid_output_pitch: f64 = 0.0;
static mut pid_output_yaw: f64 = 0.0;

static mut pid_last_roll_d_error: f64 = 0.0;
static mut pid_last_pitch_d_error: f64 = 0.0;
static mut pid_last_yaw_d_error: f64 = 0.0;

static mut gyro_roll_input: f64 = 0.0;
static mut gyro_pitch_input: f64 = 0.0;
static mut gyro_yaw_input: f64 = 0.0;

static mut gyro_angles_set:bool = false; 
fn convert(v: u8) -> f64 {
    return v as f64;
}
fn sqrt(input:f64) -> f64 {
    input.sqrt()
}
pub fn main_loop() {
    let mut loops = 0;
    let mut logger = Logger::new();
    let mut reciver_driver = ibus_receiver::new();
    let mut controller = Controller::new();
    let mut angle_pitch_acc: f64 = 0.0;
    let mut angle_roll_acc: f64 = 0.0;
    let mut angle_pitch: f64 = 0.0;
    let mut angle_roll: f64 = 0.0;
    let mut pitch_level_correction;
    let mut roll_level_correction;
    let mut start: i32 = 0;
    let mut throllite = 1000;
    let mut esc_1 = 1000.0;
    let mut esc_2 = 1000.0;
    let mut esc_3 = 1000.0;
    let mut esc_4 = 1000.0;
    let  mut acc_total_vector;
    let mut l3dgh20_driver = L3GD20H_Driver::new();
    let mut lis3dh_driver=LIS3DH_Driver::new();
    let mut PIds = config_parser::new().get_pids();
    loop {
        let now = SystemTime::now();
        let mut gyro_data = l3dgh20_driver.raw_value();
        let mut acc_data=lis3dh_driver.get_data_raw();
        let acc_x:f64= (acc_data.x as f64);
        let acc_y:f64=(acc_data.y as f64);
        let acc_z:f64=(acc_data.z as f64);
        let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();
        let gyro_roll =  (gyro_data.x as f64) ;
        let gyro_pitch = (gyro_data.y as f64);
        let gyro_yaw = (gyro_data.z  as f64) ;
        //65.5 = 1 deg/sec (check the datasheet of the MPU-6050 for mre information).
        unsafe {
            gyro_roll_input = (gyro_roll_input * 0.7) + ((gyro_roll / 65.5) * 0.3); //Gyro pid input is deg/sec.
            gyro_pitch_input = (gyro_pitch_input * 0.7) + ((gyro_pitch / 65.5) * 0.3); //Gyro pid input is deg/sec.
            gyro_yaw_input = (gyro_yaw_input * 0.7) + ((gyro_yaw / 65.5) * 0.3); //Gyro pid input is deg/sec.
        }
        //Gyro angle calculations
        //0.0000611 = 1 / (25Hz / 65.5)
        angle_pitch += gyro_pitch * 0.0000611; //Calculate the traveled pitch angle and add this to the angle_pitch variable.
        angle_roll += gyro_roll * 0.0000611; //Calculate the traveled roll angle and add this to the angle_roll variable.
        //0.000001066 = (0.0000611 * 3.142) / 180degrThe Arduino sin function is in radians
        angle_pitch -= angle_roll * sin(gyro_yaw * 0.000001066); //If the IMU has yawed transfer the roll angle to the pitch angel.
        angle_roll += angle_pitch * sin(gyro_yaw * 0.000001066); //If the IMU has yawed transfer the pitch angle to the roll angel.
        //Accelerometer angle calculations
        acc_total_vector = sqrt((acc_x * acc_x) + (acc_y * acc_y) + (acc_z * acc_z));       //Calculate the total accelerometer vector.
        if abs(acc_y) < acc_total_vector {                                        //Prevent the asin function to produce a NaN
            angle_pitch_acc = (acc_y/acc_total_vector).asin().to_radians() * 57.296;          //Calculate the pitch angle.
        }
        if abs(acc_x) < acc_total_vector {                                        //Prevent the asin function to produce a NaN
            angle_roll_acc = (acc_x/acc_total_vector).asin().to_radians() * -57.296;           //Calculate the pitch angle.
        }
        //Place the MPU-6050 spirit level and note the values in the following two lines for calibration.
        angle_pitch_acc -= 0.0;                                                   //Accelerometer calibration value for pitch.
        angle_roll_acc -= 0.0;
        angle_pitch = angle_pitch * 0.9996 + angle_pitch_acc * 0.0004; //Correct the drift of the gyro pitch angle with the accelerometer pitch angle.
        angle_roll = angle_roll * 0.9996 + angle_roll_acc * 0.0004; //Correct the drift of the gyro roll angle with the accelerometer roll angle.
        pitch_level_correction = angle_pitch * 15.0; //Calculate the pitch angle correction
        roll_level_correction = angle_roll * 15.0; //Calculate the roll angle correction
        loops += 1;
        unsafe {
            if reciver.ch6 > 1300 { start = 1; }
            if start == 1 && reciver.ch6 > 1600 {
                start = 2;
                angle_pitch = angle_pitch_acc;
                angle_roll = angle_roll_acc;
                pid_i_mem_roll = 0.0;
                pid_last_pitch_d_error = 0.0;
                pid_i_mem_pitch = 0.0;
                pid_last_pitch_d_error = 0.0;
                pid_i_mem_yaw = 0.0;
                pid_last_yaw_d_error = 0.0;
            }
            if start == 2 && reciver.ch6 < 1200 { start = 0; }
            pid_roll_setpoint = 0.0;
            if reciver.ch1 > 1508 {
                pid_roll_setpoint = (reciver.ch1 as f64 - 1508 as f64) as f64;
            } else if reciver.ch1 < 1492 {
                pid_roll_setpoint = (reciver.ch1 as f64 - 1492 as f64) as f64;
            }
            pid_roll_setpoint -= roll_level_correction;
            pid_roll_setpoint /= 3.0;
            pid_pitch_setpoint = 0.0;
            if reciver.ch1 > 1508 {
                pid_pitch_setpoint = (reciver.ch2 as f64 - 1508 as f64) as f64;
            } else if reciver.ch1 < 1492 {
                pid_pitch_setpoint = (reciver.ch2 as f64 - 1492 as f64) as f64;
            }
            pid_pitch_setpoint -= pitch_level_correction;
            pid_pitch_setpoint /= 3.0;
            pid_yaw_setpoint = 0.0;
            if reciver.ch3 > 1050 {
                if reciver.ch4 > 1508 {
                    pid_yaw_setpoint = ((reciver.ch4 as f64 - 1508 as f64) as f64) / 3.0;
                } else if reciver.ch4 < 1492 {
                    pid_yaw_setpoint = ((reciver.ch4 as f64 - 1492 as f64) as f64) / 3.0;
                }
            }
            calculate_pid(PIds.clone());
// To :DO Check it if will work with division of pid_yaw_setpoint/3.0;
            throllite = reciver.ch3;
            if start == 2 {
                if throllite > 1800 { throllite = 1800; }
                esc_1 = throllite as f64 - pid_output_pitch + pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 1 (front-right - CCW)
                esc_2 = throllite as f64 + pid_output_pitch + pid_output_roll + pid_output_yaw; //Calculate the pulse for esc 2 (rear-right - CW)
                esc_3 = throllite as f64 + pid_output_pitch - pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 3 (rear-left - CCW)
                esc_4 = throllite as f64 - pid_output_pitch - pid_output_roll + pid_output_yaw;
                if esc_1 < 1100.0 {
                    esc_1 = 1100.0;
                } //Keep the motors running.
                if esc_2 < 1100.0 {
                    esc_2 = 1100.0;
                }
                if esc_3 < 1100.0 {
                    esc_3 = 1100.0;
                }
                if esc_4 < 1100.0 {
                    esc_4 = 1100.0;
                }

                if esc_1 > 2000.0 {
                    esc_1 = 2000.0;
                }
                if esc_2 > 2000.0 {
                    esc_2 = 2000.0;
                }
                if esc_3 > 2000.0 {
                    esc_3 = 2000.0;
                }
                if esc_4 > 2000.0 {
                    esc_4 = 2000.0;
                }
            }
            else {
                esc_1 = 1000.0;
                esc_2 = 1000.0;
                esc_3 = 1000.0;
                esc_4 = 1000.0;
            }

            controller.set_throttle_external_pwm(
                esc_1 as u16,
                esc_2 as u16,
                esc_3 as u16,
                esc_4 as u16,
            );

            let time_spend = now.elapsed().unwrap().as_millis() as u128;
            let logging_data: LoggingStruct = LoggingStruct {
                acc_z,
                acc_y,
                acc_x,
                gyro_x: gyro_data.x as f64,
                gyro_y: gyro_data.y as f64,
                gyro_z: gyro_data.z as f64,
                reciver_ch1: reciver.ch1,
                reciver_ch2: reciver.ch2,
                reciver_ch3: reciver.ch3,
                reciver_ch4: reciver.ch4,
                reciver_ch5: reciver.ch5,
                reciver_ch6: reciver.ch6,
                pitch_level_correction,
                roll_level_correction,
                angle_pitch_acc,
                angle_roll_acc,
                pid_roll_setpoint,
                pid_pitch_setpoint,
                pid_yaw_setpoint,
                pid_output_roll,
                pid_output_pitch,
                esc_1,
                esc_2,
                esc_3,
                esc_4,
                temp: 0.0 as f64,
                time_spent: time_spend,
            };
            logger.write_to_log(0, &logging_data);
            logger.save_file();
        }
    }

    }


fn calculate_pid(pid: PIDS) {
    unsafe {
        //roll calculation
        pid_error_temp = gyro_roll_input - pid_roll_setpoint;
        pid_i_mem_roll += pid.roll.i * pid_error_temp;
        if pid_i_mem_roll > pid.roll.max {
            pid_i_mem_roll = pid.roll.max;
        } else if pid_i_mem_roll < (pid.roll.max * -1.0) {
            pid_i_mem_roll = pid.roll.max * -1.0;
        }
        pid_output_roll = pid.roll.p * pid_error_temp
            + pid_i_mem_roll
            + pid.roll.d * (pid_error_temp - pid_last_roll_d_error);
        if pid_output_roll > pid.roll.max {
            pid_output_roll = pid.roll.max;
        } else if pid_output_roll < (pid.roll.max * -1.0) {
            pid_output_roll = pid.roll.max * -1.0;
        }
        pid_last_roll_d_error = pid_error_temp;

        //pitch calculation
        pid_error_temp = gyro_pitch_input - pid_pitch_setpoint;
        pid_i_mem_pitch += pid.pitch.i * pid_error_temp;
        if pid_i_mem_pitch > pid.pitch.max {
            pid_i_mem_pitch = pid.pitch.max;
        } else if pid_i_mem_pitch < (pid.pitch.max * -1.0) {
            pid_i_mem_pitch = pid.pitch.max * -1.0;
        }
        pid_output_pitch = pid.pitch.p * pid_error_temp
            + pid_i_mem_pitch
            + pid.pitch.d * (pid_error_temp - pid_last_pitch_d_error);
        if pid_output_pitch > pid.pitch.max {
            pid_output_pitch = pid.pitch.max;
        } else if pid_output_pitch < (pid.pitch.max * -1.0) {
            pid_output_pitch = pid.pitch.max * -1.0;
        }
        pid_last_pitch_d_error = pid_error_temp;

        //yaw calculation
        pid_error_temp = gyro_yaw_input - pid_yaw_setpoint;
        pid_i_mem_yaw += pid.yaw.i * pid_error_temp;
        if pid_i_mem_yaw > pid.yaw.max {
            pid_i_mem_yaw = pid.yaw.max;
        } else if pid_i_mem_yaw < (pid.yaw.max * -1.0) {
            pid_i_mem_yaw = pid.yaw.max * -1.0;
        }
        pid_output_yaw = pid.yaw.p * pid_error_temp
            + pid_i_mem_yaw
            + pid.yaw.d * (pid_error_temp - pid_last_yaw_d_error);
        if pid_output_yaw > pid.yaw.max {
            pid_output_yaw = pid.yaw.max;
        } else if pid_output_yaw < (pid.yaw.max * -1.0) {
            pid_output_yaw = pid.yaw.max * -1.0;
        }
        pid_last_yaw_d_error = pid_error_temp;
    }
}
