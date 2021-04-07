const START_MOTOS_VALUE: u16 = 10;

use crate::mpu6050::Mpu6050_driver;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::Mpu6050;
extern crate pid;
use pid::Pid;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{
    thread,
    time::{self, SystemTime},
};

use crate::config_parse::*;

use crate::clk_driver::ClkDriver;
use crate::controller::*;
use crate::ibus::*;
use crate::mpu6050::*;
use crate::simple_logger;
use simple_logger::*;
fn set_bounds(mut esc_1: f64, mut esc_2: f64, mut esc_3: f64, mut esc_4: f64) {
    if esc_1 < 1100.0 {
        esc_1 = 1100.0;
    } //Keep the motors running.
    if esc_2 < 1100.0 {
        esc_2 = 1100.0;
    } //Keep the motors running.
    if esc_3 < 1100.0 {
        esc_3 = 1100.0;
    } //Keep the motors running.
    if esc_4 < 1100.0 {
        esc_4 = 1100.0;
    } //Keep the motors running.

    if esc_1 > 2000.0 {
        esc_1 = 2000.0;
    } //Limit the esc-1 pulse to 2000us.
    if esc_2 > 2000.0 {
        esc_2 = 2000.0;
    } //Limit the esc-2 pulse to 2000us.
    if esc_3 > 2000.0 {
        esc_3 = 2000.0;
    } //Limit the esc-3 pulse to 2000us.
    if esc_4 > 2000.0 {
        esc_4 = 2000.0;
    }
}

pub fn main_loop() {
    let mut loops = 0;
    let mut reciver_driver = ibus_receiver::new();
    let mut mpu6050 = Mpu6050_driver::new();
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

    let mut throllite;
    let mut esc_1 = 1000.0;
    let mut esc_2 = 1000.0;
    let mut esc_3 = 1000.0;
    let mut esc_4 = 1000.0;
    let autolevel = config.auto_level_config();
    simple_logger::write_log(
        LevelOfLog::INFO,
        "CREATE DRIVER OBJECTS FD4522e :".parse().unwrap(),
    );

    let pid_settings = config.get_pids();
    let mut pid_roll = Pid::new(
        pid_settings.roll.p as f64,
        pid_settings.roll.i as f64,
        pid_settings.roll.d as f64,
        pid_settings.roll.max as f64,
        pid_settings.roll.max as f64,
        pid_settings.roll.max as f64,
        0.0,
    );

    let mut pid_pitch = Pid::new(
        pid_settings.pitch.p as f64,
        pid_settings.pitch.i as f64,
        pid_settings.pitch.d as f64,
        pid_settings.pitch.max as f64,
        pid_settings.pitch.max as f64,
        pid_settings.pitch.max as f64,
        0.0,
    );
    let mut pid_yaw = Pid::new(
        pid_settings.yaw.p as f64,
        pid_settings.yaw.i as f64,
        pid_settings.yaw.d as f64,
        pid_settings.yaw.max as f64,
        pid_settings.yaw.max as f64,
        pid_settings.yaw.max as f64,
        0.0,
    );

    /* init*/
    loop {
        let gyro_values = mpu6050.get_gyro_values(1);
        simple_logger::write_log(LevelOfLog::INFO, "GYRO DATA:".parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, gyro_values.x.to_string());
        simple_logger::write_log(LevelOfLog::INFO, gyro_values.y.to_string());
        simple_logger::write_log(LevelOfLog::INFO, gyro_values.z.to_string());
        clk_driver.set_pin_clk_high();
        let now = SystemTime::now();

        let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();
        simple_logger::write_log(LevelOfLog::INFO, "READ DATA FROM RC :".parse().unwrap());
        //set_bounds(esc_1,esc_2,esc_3,esc_4);
        simple_logger::write_log(LevelOfLog::INFO, "LIST SETTINGS :".parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, autolevel.to_string().parse().unwrap());

        let acc_value = mpu6050.get_acc_values(1);

        let acc_x = acc_value.x;
        let acc_y = acc_value.y;
        let acc_z = acc_value.z;

        simple_logger::write_log(LevelOfLog::INFO, "start".to_string());
        simple_logger::write_log(LevelOfLog::INFO, start.to_string().parse().unwrap());

        /*

                //Gyro angle calculations
        //0.0000611 = 1 / (250Hz / 65.5)
        angle_pitch += gyro_pitch * 0.0000611;                                    //Calculate the traveled pitch angle and add this to the angle_pitch variable.
        angle_roll += gyro_roll * 0.0000611;                                      //Calculate the traveled roll angle and add this to the angle_roll variable.

        //0.000001066 = 0.0000611 * (3.142(PI) / 180degr) The Arduino sin function is in radians
        angle_pitch -= angle_roll * sin(gyro_yaw * 0.000001066);                  //If the IMU has yawed transfer the roll angle to the pitch angel.
        angle_roll += angle_pitch * sin(gyro_yaw * 0.000001066);                  //If the IMU has yawed transfer the pitch angle to the roll angel.

               */

        angle_pitch += (gyro_values.y) * 0.004580152671755725; //Calculate the traveled pitch angle and add this to the angle_pitch variable.
        angle_roll += (gyro_values.x) * 0.004580152671755725;

        //0.000001066 = 0.0000611 * (3.142(PI) / 180degr) The Arduino sin function is in radians
        angle_pitch -= angle_roll * (gyro_values.z * 0.0000799387443661525).sin(); //If the IMU has yawed transfer the roll angle to the pitch angel.
        angle_roll += angle_pitch * (gyro_values.z * 0.0000799387443661525).sin();

        // acc_total_vector = sqrt((acc_x*acc_x)+(acc_y*acc_y)+(acc_z*acc_z));
        let acc_total_vector = ((acc_x * acc_x) + (acc_y * acc_y) + (acc_z * acc_z) as f64).sqrt();

        simple_logger::write_log(LevelOfLog::INFO, "acc_total_vector".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            acc_total_vector.to_string().parse().unwrap(),
        );

        simple_logger::write_log(LevelOfLog::INFO, "acc_z".to_string());
        simple_logger::write_log(LevelOfLog::INFO, acc_z.to_string().parse().unwrap());

        simple_logger::write_log(LevelOfLog::INFO, "acc_x".to_string());
        simple_logger::write_log(LevelOfLog::INFO, acc_x.to_string().parse().unwrap());

        simple_logger::write_log(LevelOfLog::INFO, "acc_y".to_string());
        simple_logger::write_log(LevelOfLog::INFO, acc_y.to_string().parse().unwrap());

        simple_logger::write_log(LevelOfLog::INFO, "angle_pitch_acc".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            angle_pitch_acc.to_string().parse().unwrap(),
        );

        /*

                if(abs(acc_y) < acc_total_vector){                                        //Prevent the asin function to produce a NaN
          angle_pitch_acc = asin((float)acc_y/acc_total_vector)* 57.296;          //Calculate the pitch angle.
        }
        if(abs(acc_x) < acc_total_vector){                                        //Prevent the asin function to produce a NaN
          angle_roll_acc = asin((float)acc_x/acc_total_vector)* -57.296;          //Calculate the roll angle.
        }
               */
        if acc_y.abs() < acc_total_vector {
            angle_pitch_acc = (acc_y / acc_total_vector).asin() * 57.296;
        }

        if acc_x.abs() < acc_total_vector {
            angle_roll_acc = (acc_x / acc_total_vector).asin() * -57.296;
        }

        simple_logger::write_log(LevelOfLog::INFO, "angle_roll_acc".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            angle_roll_acc.to_string().parse().unwrap(),
        );
        //x -= y; // equivalent to the expression x = x - y;
        angle_pitch_acc = angle_pitch_acc - 0.0;
        angle_roll_acc = angle_roll_acc - 0.0;

        angle_pitch = angle_pitch * 0.9996 + angle_pitch_acc * 0.0004; //Correct the drift of the gyro pitch angle with the accelerometer pitch angle.
        angle_roll = angle_roll * 0.9996 + angle_roll_acc * 0.0004; //Correct the drift of the gyro roll angle with the accelerometer roll angle.

        simple_logger::write_log(LevelOfLog::INFO, "angle_pitch".to_string());
        simple_logger::write_log(LevelOfLog::INFO, angle_pitch.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, "angle_roll_acc".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            angle_roll_acc.to_string().parse().unwrap(),
        );
        pitch_level_correction = (angle_pitch * 15.0) as f64; //Calculate the pitch angle correction
        roll_level_correction = (angle_roll * 15.0) as f64; //Calculate the roll angle correction
        simple_logger::write_log(LevelOfLog::INFO, "angle_pitch".to_string());
        simple_logger::write_log(LevelOfLog::INFO, angle_pitch.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, "angle_roll".to_string());
        simple_logger::write_log(LevelOfLog::INFO, angle_roll.to_string().parse().unwrap());

        if autolevel == 0 {
            //If the quadcopter is not in auto-level mode
            pitch_level_correction = 0.0; //Set the pitch angle correction to zero.
            roll_level_correction = 0.0; //Set the roll angle correcion to zero.
        }





        loops = loops + 1;

        simple_logger::write_log(LevelOfLog::INFO, "loops".to_string());
        simple_logger::write_log(LevelOfLog::INFO, loops.to_string().parse().unwrap());

        if reciver.ch6 > 1900 {
            /*
            controller.turn_motor(Channel::C0, 1000);
            controller.turn_motor(Channel::C1, 1000);
            controller.turn_motor(Channel::C2, 1000);
            controller.turn_motor(Channel::C3, 1000);
            */

            start = 2;
            angle_pitch = angle_pitch_acc; //Set the gyro pitch angle equal to the accelerometer pitch angle when the quadcopter is started.
            angle_roll = angle_roll_acc;
            //gyro_angles_set=true;
            pid_roll.reset_integral_term();
            pid_pitch.reset_integral_term();
            pid_yaw.reset_integral_term();
            simple_logger::write_log(LevelOfLog::INFO, "angle_pitch_acc".to_string());
            simple_logger::write_log(
                LevelOfLog::INFO,
                angle_pitch_acc.to_string().parse().unwrap(),
            );
            simple_logger::write_log(LevelOfLog::INFO, "angle_roll_acc".to_string());
            simple_logger::write_log(
                LevelOfLog::INFO,
                angle_roll_acc.to_string().parse().unwrap(),
            );
            simple_logger::write_log(LevelOfLog::INFO, format!("{}", "Unlocked 1"));
        }

        if start == 2 && reciver.ch6 < 1050 {
            start = 0;
            simple_logger::write_log(LevelOfLog::INFO, format!("{}", "Blocked 3"));
        }

        pid_roll.setpoint = 0.0;
        //We need a little dead band of 16us for better results.
        if reciver.ch1 > 1508 {
            pid_roll.setpoint = reciver.ch1 as f64 - 1508.0;
        } else if reciver.ch1 < 1492 {
            pid_roll.setpoint = reciver.ch1 as f64 - 1492.0;
        }

        pid_roll.setpoint = pid_roll.setpoint - roll_level_correction; //Subtract the angle correction from the standardized receiver roll input value.
                                                                       //x = x / y;

        pid_roll.setpoint = pid_roll.setpoint / 3.0;
        simple_logger::write_log(LevelOfLog::INFO, "pid_roll.setpoint".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            pid_roll.setpoint.to_string().parse().unwrap(),
        );
        pid_pitch.setpoint = 0.0;
        //We need a little dead band of 16us for better results.
        if reciver.ch2 > 1508 {
            pid_pitch.setpoint = reciver.ch2 as f64 - 1508.0;
        } else if reciver.ch2 < 1492 {
            pid_pitch.setpoint = reciver.ch2 as f64 - 1492.0;
        }

        pid_pitch.setpoint = pid_pitch.setpoint - pitch_level_correction; //Subtract the angle correction from the standardized receiver pitch input value.
        pid_pitch.setpoint = pid_pitch.setpoint / 3.0;
        simple_logger::write_log(LevelOfLog::INFO, "pid_pitch.setpoint".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            pid_pitch.setpoint.to_string().parse().unwrap(),
        );
        pid_yaw.setpoint = 0.0;
        //We need a little dead band of 16us for better results.
        if reciver.ch3 > 1050 {
            //Do not yaw when turning off the motors.
            if reciver.ch4 > 1508 {
                pid_yaw.setpoint = (reciver.ch4 as f64 - 1508.0) / 3.0 as f64;
            } else if reciver.ch4 < 1492 {
                pid_yaw.setpoint = (reciver.ch4 as f64 - 1492.0) / 3.0 as f64;
            }
        }

        let pid_output_roll = pid_roll
            .next_control_output((roll_level_correction - pid_roll.setpoint) as f64)
            .output;

        simple_logger::write_log(LevelOfLog::INFO, "pid_output_roll".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            pid_output_roll.to_string().parse().unwrap(),
        );
        let pid_output_pitch = pid_pitch
            .next_control_output((pitch_level_correction - pid_pitch.setpoint) as f64)
            .output;
        let pid_output_yaw = pid_yaw.next_control_output(pid_yaw.setpoint).output;

        simple_logger::write_log(LevelOfLog::INFO, "pid_output_pitch".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            pid_output_pitch.to_string().parse().unwrap(),
        );

        simple_logger::write_log(LevelOfLog::INFO, "pid_output_yaw".to_string());
        simple_logger::write_log(
            LevelOfLog::INFO,
            pid_output_yaw.to_string().parse().unwrap(),
        );
        throllite = reciver.ch3;
        simple_logger::write_log(LevelOfLog::INFO, "throllite".to_string());
        simple_logger::write_log(LevelOfLog::INFO, throllite.to_string().parse().unwrap());
        if start == 2 {
            if throllite > 1800 {
                throllite = 1800;
            }
            esc_1 = throllite as f64 - pid_output_pitch + pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 1 (front-right - CCW)
            esc_2 = throllite as f64 + pid_output_pitch + pid_output_roll + pid_output_yaw; //Calculate the pulse for esc 2 (rear-right - CW)
            esc_3 = throllite as f64 + pid_output_pitch - pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 3 (rear-left - CCW)
            esc_4 = throllite as f64 - pid_output_pitch - pid_output_roll + pid_output_yaw; //Calculate the pulse for esc 4 (front-left - CW)
            simple_logger::write_log(LevelOfLog::INFO, "esc_1".to_string());
            simple_logger::write_log(LevelOfLog::INFO, esc_1.to_string().parse().unwrap());
            simple_logger::write_log(LevelOfLog::INFO, "esc_2".to_string());
            simple_logger::write_log(LevelOfLog::INFO, esc_2.to_string().parse().unwrap());
            simple_logger::write_log(LevelOfLog::INFO, "esc_3".to_string());
            simple_logger::write_log(LevelOfLog::INFO, esc_3.to_string().parse().unwrap());
            simple_logger::write_log(LevelOfLog::INFO, "esc_4".to_string());
            simple_logger::write_log(LevelOfLog::INFO, esc_4.to_string().parse().unwrap());

            if esc_1 < 1100.0 {
                esc_1 = 1100.0;
            } //Keep the motors running.
            if esc_2 < 1100.0 {
                esc_2 = 1100.0;
            } //Keep the motors running.
            if esc_3 < 1100.0 {
                esc_3 = 1100.0;
            } //Keep the motors running.
            if esc_4 < 1100.0 {
                esc_4 = 1100.0;
            } //Keep the motors running.

            if esc_1 > 2000.0 {
                esc_1 = 2000.0;
            } //Limit the esc-1 pulse to 2000us.
            if esc_2 > 2000.0 {
                esc_2 = 2000.0;
            } //Limit the esc-2 pulse to 2000us.
            if esc_3 > 2000.0 {
                esc_3 = 2000.0;
            } //Limit the esc-3 pulse to 2000us.
            if esc_4 > 2000.0 {
                esc_4 = 2000.0;
            }
        } else {
            esc_1 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-1.
            esc_2 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-2.
            esc_3 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-3.
            esc_4 = 1000.0; //If start is not 2 keep a 1000us pulse for ess-4.
        }

        simple_logger::write_log(LevelOfLog::INFO, "esc_1".to_string());
        simple_logger::write_log(LevelOfLog::INFO, esc_1.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, "esc_2".to_string());
        simple_logger::write_log(LevelOfLog::INFO, esc_2.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, "esc_3".to_string());
        simple_logger::write_log(LevelOfLog::INFO, esc_3.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, "esc_4".to_string());
        simple_logger::write_log(LevelOfLog::INFO, esc_4.to_string().parse().unwrap());
        set_bounds(esc_1, esc_2, esc_3, esc_4);
        controller.set_throttle_external_pwm(
            esc_1 as u16,
            esc_2 as u16,
            esc_3 as u16,
            esc_4 as u16,
        );

        simple_logger::write_log(LevelOfLog::INFO, format!("{} \n", "Time spent:"));
        simple_logger::write_log(
            LevelOfLog::INFO,
            format!("{}", now.elapsed().expect("err").as_millis()),
        );
        /*
        set_throttle_external_pwm(esc_1 as u16, esc_2 as u16, esc_3 as u16, esc_4 as u16);
        controller.turn_motor(Channel::C0, esc_1 as u16);
        controller.turn_motor(Channel::C1,  esc_2 as u16);
        controller.turn_motor(Channel::C2, esc_3 as u16);
        controller.turn_motor(Channel::C3, esc_4 as u16);
        */
        clk_driver.set_pin_clk_low();
    }
}
