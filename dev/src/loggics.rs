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

use crate::l3dgh20::L3GD20H_Driver;
use crate::untils::sin;
use cgmath::Zero;
use std::borrow::Borrow;

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

pub fn main_loop() {
    let mut loops = 0;
    let mut logger = Logger::new();
    let mut reciver_driver = ibus_receiver::new();
    let mut controller = Controller::new();
    let mut config = config_parser::new();
    let mut clk_driver = ClkDriver::new();
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
    let autolevel = config.auto_level_config();
    let pid_settings = config.get_pids();
    let mut pid_roll = Pid::new(
        pid_settings.roll.p as f64,
        pid_settings.roll.i as f64,
        pid_settings.roll.d as f64,
        pid_settings.roll.max as f64,
        pid_settings.roll.max as f64,
        pid_settings.roll.max as f64,
        5.0,
    );

    let mut pid_pitch = Pid::new(
        pid_settings.pitch.p as f64,
        pid_settings.pitch.i as f64,
        pid_settings.pitch.d as f64,
        pid_settings.pitch.max as f64,
        pid_settings.pitch.max as f64,
        pid_settings.pitch.max as f64,
        5.0,
    );
    let mut pid_yaw = Pid::new(
        pid_settings.yaw.p as f64,
        pid_settings.yaw.i as f64,
        pid_settings.yaw.d as f64,
        pid_settings.yaw.max as f64,
        pid_settings.yaw.max as f64,
        pid_settings.yaw.max as f64,
        5.0,
    );
    /* init*/
    clk_driver.set_pin_clk_high();

    let mut l3dgh20_driver = L3GD20H_Driver::new();

    loop {
        unsafe {
            let now = SystemTime::now();
            let mut gyro_data = l3dgh20_driver.raw_value();
            let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();

            let gyro_roll = gyro_data.x as f64;
            let gyro_pitch = gyro_data.y as f64;
            let gyro_yaw = gyro_data.z as f64;

            //65.5 = 1 deg/sec (check the datasheet of the MPU-6050 for more information).
            gyro_roll_input = (gyro_roll_input * 0.7) + ((gyro_roll / 65.5) * 0.3); //Gyro pid input is deg/sec.
            gyro_pitch_input = (gyro_pitch_input * 0.7) + ((gyro_pitch / 65.5) * 0.3); //Gyro pid input is deg/sec.
            gyro_yaw_input = (gyro_yaw_input * 0.7) + ((gyro_yaw / 65.5) * 0.3); //Gyro pid input is deg/sec.

            //Gyro angle calculations
            //0.0000611 = 1 / (250Hz / 65.5)
            angle_pitch += gyro_pitch * 0.0000611; //Calculate the traveled pitch angle and add this to the angle_pitch variable.
            angle_roll += gyro_roll * 0.0000611; //Calculate the traveled roll angle and add this to the angle_roll variable.

            //0.000001066 = 0.0000611 * (3.142(PI) / 180degr) The Arduino sin function is in radians
            angle_pitch -= angle_roll * sin(gyro_yaw * 0.000001066); //If the IMU has yawed transfer the roll angle to the pitch angel.
            angle_roll += angle_pitch * sin(gyro_yaw * 0.000001066); //If the IMU has yawed transfer the pitch angle to the roll angel.

            /*

                    angle_pitch += (gyro_values.y) * 0.00016795; //Calculate the traveled pitch angle and add this to the angle_pitch variable.
                    angle_roll += (gyro_values.x) * 0.00016795;

                    angle_pitch -= angle_roll * (gyro_values.z * 0.00000293166).sin(); //If the IMU has yawed transfer the roll angle to the pitch angel.
                    angle_roll += angle_pitch * (gyro_values.z * 0.00000293166).sin();

            */

            angle_pitch_acc = 0.0;
            angle_roll_acc = 0.0;

            angle_pitch = angle_pitch * 0.9996 + angle_pitch_acc * 0.0004; //Correct the drift of the gyro pitch angle with the accelerometer pitch angle.
            angle_roll = angle_roll * 0.9996 + angle_roll_acc * 0.0004; //Correct the drift of the gyro roll angle with the accelerometer roll angle.

            pitch_level_correction = angle_pitch * 15.0; //Calculate the pitch angle correction
            roll_level_correction = angle_roll * 15.0; //Calculate the roll angle correction

            if (autolevel.is_zero()) {
                //If the quadcopter is not in auto-level mode
                pitch_level_correction = 0.0; //Set the pitch angle correction to zero.
                roll_level_correction = 0.0; //Set the roll angle correcion to zero.
            }

            loops += 1;
            //For starting the motors: throttle low and yaw left (step 1).
            if reciver.ch3 < 1050 && reciver.ch4 < 1050 {
                start = 1
            }
            //When yaw stick is back in the center position start the motors (step 2).
            if start == 1 && reciver.ch3 < 1050 && reciver.ch4 > 1450 {
                start = 2;
                angle_pitch = angle_pitch_acc; //Set the gyro pitch angle equal to the accelerometer pitch angle when the quadcopter is started.
                angle_roll = angle_roll_acc; //Set the gyro roll angle equal to the accelerometer roll angle when the quadcopter is started.
                                             //Set the IMU started flag.
                                             
            }

            if start == 2 && reciver.ch3 < 1050 && reciver.ch4 > 1950 {
                start = 0;
            }

            pid_roll.setpoint = 0.0;
            //We need a little dead band of 16us for better results.
            if reciver.ch1 > 1508 {
                pid_roll.setpoint = reciver.ch1 as f64 - 1508.0;
            } else if reciver.ch1 < 1492 {
                pid_roll.setpoint = reciver.ch1 as f64 - 1492.0;
            }
            pid_roll.setpoint -= roll_level_correction; //Subtract the angle correction from the standardized receiver roll input value
            pid_roll.setpoint = pid_roll.setpoint / 3.0;

            pid_pitch.setpoint = 0.0;
            //We need a little dead band of 16us for better results.
            if reciver.ch2 > 1508 {
                pid_pitch.setpoint = reciver.ch2 as f64 - 1508.0;
            } else if reciver.ch2 < 1492 {
                pid_pitch.setpoint = reciver.ch2 as f64 - 1492.0;
            }

            pid_pitch.setpoint -= pitch_level_correction; //Subtract the angle correction from the standardized receiver pitch input value.
            pid_pitch.setpoint /= 3.0;
            pid_yaw.setpoint = 0.0;
            //We need a little dead band of 16us for better results.
            if reciver.ch3 > 1050 {
                //Do not yaw when turning off the motors.
                if reciver.ch4 > 1508 {
                    pid_yaw.setpoint = ((reciver.ch4 as f64 - 1508.0) / 3.0) as f64;
                } else if reciver.ch4 < 1492 {
                    pid_yaw.setpoint = ((reciver.ch4 as f64 - 1492.0) / 3.0) as f64;
                }
            }
            pid_output_roll = pid_roll
                .next_control_output((gyro_roll_input - pid_roll.setpoint) as f64)
                .output;

            pid_output_pitch = pid_pitch
                .next_control_output((gyro_pitch_input - pid_pitch.setpoint) as f64)
                .output;
            pid_output_yaw = pid_yaw
                .next_control_output((gyro_yaw_input - pid_yaw.setpoint) as f64)
                .output;

            throllite = reciver.ch3;

            esc_1 = throllite as f64 - pid_output_pitch + pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 1 (front-right - CCW)
            esc_2 = throllite as f64 + pid_output_pitch + pid_output_roll + pid_output_yaw; //Calculate the pulse for esc 2 (rear-right - CW)
            esc_3 = throllite as f64 + pid_output_pitch - pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 3 (rear-left - CCW)
            esc_4 = throllite as f64 - pid_output_pitch - pid_output_roll + pid_output_yaw;
            //Calculate the pulse for esc 4 (front-left - CW)

            if start == 2 {
                if throllite > 1800 {
                    throllite = 1800;
                }

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
            } else {
                esc_1 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-1.
                esc_2 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-2.
                esc_3 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-3.
                esc_4 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-4.
            }
            controller.set_throttle_external_pwm(
                esc_1 as u16,
                esc_2 as u16,
                esc_3 as u16,
                esc_4 as u16,
            );

            let time_spend = now.elapsed().unwrap().as_millis() as u128;
            let logging_data: LoggingStruct = LoggingStruct {
                acc_z: 0.0,
                acc_y: 0.0,
                acc_x: 0.0,
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
                pid_roll_setpoint: pid_roll.setpoint,
                pid_pitch_setpoint: pid_pitch.setpoint,
                pid_yaw_setpoint: pid_yaw.setpoint,
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
        if (pid_i_mem_roll > pid.roll.max) {
            pid_i_mem_roll = pid.roll.max;
        } else if (pid_i_mem_roll < (pid.roll.max * -1.0)) {
            pid_i_mem_roll = pid.roll.max * -1.0;
        }
        pid_output_roll = pid.roll.p * pid_error_temp
            + pid_i_mem_roll
            + pid.roll.d * (pid_error_temp - pid_last_roll_d_error);
        if (pid_output_roll > pid.roll.max) {
            pid_output_roll = pid.roll.max;
        } else if (pid_output_roll < (pid.roll.max * -1.0)) {
            pid_output_roll = pid.roll.max * -1.0;
        }
        pid_last_roll_d_error = pid_error_temp;

        //pitch calculation
        pid_error_temp = gyro_pitch_input - pid_pitch_setpoint;
        pid_i_mem_pitch += pid.pitch.i * pid_error_temp;
        if (pid_i_mem_pitch > pid.pitch.max) {
            pid_i_mem_pitch = pid.pitch.max;
        } else if (pid_i_mem_pitch < (pid.pitch.max * -1.0)) {
            pid_i_mem_pitch = pid.pitch.max * -1.0;
        }
        pid_output_pitch = pid.pitch.p * pid_error_temp
            + pid_i_mem_pitch
            + pid.pitch.d * (pid_error_temp - pid_last_pitch_d_error);
        if (pid_output_pitch > pid.pitch.max) {
            pid_output_pitch = pid.pitch.max;
        } else if (pid_output_pitch < (pid.pitch.max * -1.0)) {
            pid_output_pitch = pid.pitch.max * -1.0;
        }
        pid_last_pitch_d_error = pid_error_temp;

        //yaw calculation
        pid_error_temp = gyro_yaw_input - pid_yaw_setpoint;
        pid_i_mem_yaw += pid.yaw.i * pid_error_temp;
        if (pid_i_mem_yaw > pid.yaw.max) {
            pid_i_mem_yaw = pid.yaw.max;
        } else if (pid_i_mem_yaw < (pid.yaw.max * -1.0)) {
            pid_i_mem_yaw = pid.yaw.max * -1.0;
        }
        pid_output_yaw = pid.yaw.p * pid_error_temp
            + pid_i_mem_yaw
            + pid.yaw.d * (pid_error_temp - pid_last_yaw_d_error);
        if (pid_output_yaw > pid.yaw.max) {
            pid_output_yaw = pid.yaw.max;
        } else if (pid_output_yaw < (pid.yaw.max * -1.0)) {
            pid_output_yaw = pid.yaw.max * -1.0;
        }
        pid_last_yaw_d_error = pid_error_temp;
    }
}
