#[allow(non_camel_case_types)]
/*
This code is a port of our verison of YMFC-AL
 YMFC-AL -> Electrics Eagles (Arduino) -> PiElectricsEagles (Raspberry Pi Zero )
*/
use crate::config_parse::*;
use crate::controller::*;
use crate::filter::ABfilter;
use crate::filter::Filter;
use crate::ibus::*;
use crate::imu::imu;
use crate::logger::*;
use crate::utils::abs;
use crate::utils::sin;
use core::time;
use std::thread;
use std::time::SystemTime;
static mut pid_error_temp: f32 = 0.0;

static mut pid_i_mem_roll: f32 = 0.0;
static mut pid_i_mem_pitch: f32 = 0.0;
static mut pid_i_mem_yaw: f32 = 0.0;

static mut pid_roll_setpoint: f32 = 0.0;
static mut pid_pitch_setpoint: f32 = 0.0;
static mut pid_yaw_setpoint: f32 = 0.0;

static mut pid_output_roll: f32 = 0.0;
static mut pid_output_pitch: f32 = 0.0;
static mut pid_output_yaw: f32 = 0.0;

static mut pid_last_roll_d_error: f32 = 0.0;
static mut pid_last_pitch_d_error: f32 = 0.0;
static mut pid_last_yaw_d_error: f32 = 0.0;

static mut gyro_roll_input: f32 = 0.0;
static mut gyro_pitch_input: f32 = 0.0;
static mut gyro_yaw_input: f32 = 0.0;
const a: f32 = 2.0;
const b: f32 = 0.7;

fn sqrt(input: f32) -> f32 {
    input.sqrt()
}
pub fn main_loop() {
    let init_throllite=1000;
    let mut logger = Logger::new();
    let mut reciver_driver = ibus_receiver::new();
    let mut controller = Controller::new();
    let mut angle_pitch_acc: f32 = 0.0;
    let mut angle_roll_acc: f32 = 0.0;
    let mut angle_pitch: f32 = 0.0;
    let mut angle_roll: f32 = 0.0;
    let mut pitch_level_correction;
    let mut roll_level_correction;
    let mut start: i32 = 0;
    let mut throttle: u16;
    let mut esc_1: f32;
    let mut esc_2: f32;
    let mut esc_3: f32;
    let mut esc_4: f32;
    let mut acc_total_vector;
    let mut config = config_parser::new();
    controller.set_throttle_external_pwm(init_throllite,init_throllite,init_throllite,init_throllite);

    let PIds = config.get_pids();

    let mut imu =imu::new();
    thread::sleep(time::Duration::from_millis(5000));
    println!("Calibrate Gyro . Do not touch drone including squrrels");
    imu.calibrate();

    thread::sleep(time::Duration::from_millis(1000));
    println!("Initialize all devices finished!!! Welcome to PIEEA V2");
    loop {
        let now = SystemTime::now();
        let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();

        /*let raw_gyro_roll = ABfilter(gyro.roll as f32, a, b);
        let raw_gyro_pitch = ABfilter(gyro.pitch as f32, a, b);
        let raw_gyro_yaw = ABfilter(gyro.yaw as f32, a, b);

        let gyro_roll = ABfilter(raw_gyro_roll, a, b);
        let gyro_pitch = ABfilter(raw_gyro_pitch, a, b);
        let gyro_yaw = ABfilter(raw_gyro_yaw, a, b);*/
        let gyro_data=imu.get_normalised_gyro_data();
        let acc_data=imu.get_acc_data();


        /*

        roll=x
        pitch = y
        yaw = z


                 */
        let gyro_roll =  gyro_data.pitch;
        let gyro_pitch = gyro_data.roll;
        let gyro_yaw =  (gyro_data.yaw)*-1;

        let acc_x: f32 = (acc_data.roll as f32)*-1.0;
        let acc_y: f32 = acc_data.pitch as f32;
        let acc_z: f32 = acc_data.yaw as f32;

        //65.5 = 1 deg/sec (check the datasheet of the MPU-6050 for mre information).
        unsafe {
            gyro_roll_input = (gyro_roll_input * 0.7) + ((gyro_roll as f32 / 65.5) * 0.3); //Gyro pid input is deg/sec.
            gyro_pitch_input = (gyro_pitch_input * 0.7) + ((gyro_pitch as f32 / 65.5) * 0.3); //Gyro pid input is deg/sec.
            gyro_yaw_input = (gyro_yaw_input * 0.7) + ((gyro_yaw as f32 / 65.5) * 0.3);
            //Gyro pid input is deg/sec.
        }
        //Gyro angle calculations
        //0.0000611 = 1 / (25Hz / 65.5)
        angle_pitch += gyro_pitch as f32 * 0.0000916; //Calculate the traveled pitch angle and add this to the angle_pitch variable.
        angle_roll += gyro_roll as f32 * 0.0000916; //Calculate the traveled roll angle and add this to the angle_roll variable.
                                             //0.000001066 = (0.0000611 * 3.142) / 180degrThe Arduino sin function is in radians
        angle_pitch -= angle_roll * sin(gyro_yaw as f64 * 0.000001599) as f32; //If the IMU has yawed transfer the roll angle to the pitch angel.
        angle_roll += angle_pitch * sin(gyro_yaw as f64 * 0.000001599) as f32; //If the IMU has yawed transfer the pitch angle to the roll angel.
                                                                               //Accelerometer angle calculations
        acc_total_vector = sqrt((acc_x * acc_x) + (acc_y * acc_y) + (acc_z * acc_z)); //Calculate the total accelerometer vector.
        if abs(acc_y) < acc_total_vector {
            //Prevent the asin function to produce a NaN
            angle_pitch_acc = (acc_y / acc_total_vector).asin() * 57.296; //Calculate the pitch angle.
        }
        if abs(acc_x) < acc_total_vector {
            //Prevent the asin function to produce a NaN
            angle_roll_acc = (acc_x / acc_total_vector).asin() * -57.296; //Calculate the pitch angle.
        }
        //Place the MPU-6050 spirit level and note the values in the following two lines for calibration.
        angle_pitch_acc -= 0.0; //Accelerometer calibration value for pitch.
        angle_roll_acc -= 0.0;
        angle_pitch = angle_pitch * 0.9996 + angle_pitch_acc * 0.0004; //Correct the drift of the gyro pitch angle with the accelerometer pitch angle.
        angle_roll = angle_roll * 0.9996 + angle_roll_acc * 0.0004; //Correct the drift of the gyro roll angle with the accelerometer roll angle.
        pitch_level_correction = angle_pitch * 0.0; //Calculate the pitch angle correction
        roll_level_correction = angle_roll * 0.0; //Calculate the roll angle correction
        unsafe {
            if reciver.ch5 > 1450 && reciver.ch6 < 1050 {
                start = 1;
                println!("unlocked #1");
            }
            if start == 1 && reciver.ch6 > 1450 && reciver.ch5 > 1450 {
                start = 2;

                println!("unlocked #2");
                angle_pitch = angle_pitch_acc;
                angle_roll = angle_roll_acc;
                pid_i_mem_roll = 0.0;
                pid_last_pitch_d_error = 0.0;
                pid_i_mem_pitch = 0.0;
                pid_last_pitch_d_error = 0.0;
                pid_i_mem_yaw = 0.0;
                pid_last_yaw_d_error = 0.0;
            }
            if start == 2 && reciver.ch3 < 1050 && reciver.ch4 > 1950 {
                start = 0;
            }

            pid_roll_setpoint = 0.0;
            if reciver.ch1 > 1508 {
                pid_roll_setpoint = (reciver.ch1 as f32 - 1508 as f32) as f32;
            } else if reciver.ch1 < 1492 {
                pid_roll_setpoint = (reciver.ch1 as f32 - 1492 as f32) as f32;
            }
            pid_roll_setpoint -= roll_level_correction;
            pid_roll_setpoint /= 3.0;

            pid_pitch_setpoint = 0.0;
            if reciver.ch2 > 1508 {
                pid_pitch_setpoint = (reciver.ch2 as f32 - 1508 as f32) as f32;
            } else if reciver.ch2 < 1492 {
                pid_pitch_setpoint = (reciver.ch2 as f32 - 1492 as f32) as f32;
            }
            pid_pitch_setpoint -= pitch_level_correction;
            pid_pitch_setpoint /= 3.0;

            pid_yaw_setpoint = 0.0;
            if reciver.ch3 > 1050 {
                if reciver.ch4 > 1508 {
                    pid_yaw_setpoint = (reciver.ch4 as f32 - 1508 as f32) / 3.0;
                } else if reciver.ch4 < 1492 {
                    pid_yaw_setpoint = (reciver.ch4 as f32 - 1492 as f32) / 3.0;
                }
            }

            calculate_pid(PIds.clone());

            throttle = reciver.ch3;

            if start == 2 {
                if throttle > 1800 {
                    throttle = 1800;
                }
                esc_1 = throttle as f32 - pid_output_pitch + pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 1 (front-right - CCW)
                esc_2 = throttle as f32 + pid_output_pitch + pid_output_roll + pid_output_yaw; //Calculate the pulse for esc 2 (rear-right - CW)
                esc_3 = throttle as f32 + pid_output_pitch - pid_output_roll - pid_output_yaw; //Calculate the pulse for esc 3 (rear-left - CCW)
                esc_4 = throttle as f32 - pid_output_pitch - pid_output_roll + pid_output_yaw;
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
                acc_x: acc_x as i32,
                acc_y: acc_y as i32,
                acc_z: acc_z as i32,
                gyro_x: gyro_roll as i32,
                gyro_y: gyro_pitch as i32,
                gyro_z: gyro_yaw as i32,
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
                temp: 0.0,
                time_spent: time_spend,
            };
            logger.write_to_log(0, &logging_data);
            //logger.print_telemetry();
            logger.save_file();
        }
    }
}

fn calculate_pid(pid: PIDS) {
    unsafe {
        //roll calculation
        pid_error_temp = gyro_roll_input - pid_roll_setpoint;
        pid_i_mem_roll += pid.roll.i as f32 * pid_error_temp;
        if pid_i_mem_roll > pid.roll.max as f32 {
            pid_i_mem_roll = pid.roll.max as f32;
        } else if pid_i_mem_roll < (pid.roll.max as f32 * -1.0) {
            pid_i_mem_roll = pid.roll.max as f32 * -1.0;
        }

        pid_output_roll = pid.roll.p as f32 * pid_error_temp
            + pid_i_mem_roll
            + pid.roll.d as f32 * (pid_error_temp - pid_last_roll_d_error);

        if pid_output_roll > pid.roll.max as f32 {
            pid_output_roll = pid.roll.max as f32;
        } else if pid_output_roll < (pid.roll.max as f32 * -1.0) {
            pid_output_roll = pid.roll.max as f32 * -1.0;
        }
        pid_last_roll_d_error = pid_error_temp;

        //pitch calculation
        pid_error_temp = gyro_pitch_input - pid_pitch_setpoint;
        pid_i_mem_pitch += pid.pitch.i as f32 * pid_error_temp;
        if pid_i_mem_pitch > pid.pitch.max as f32 {
            pid_i_mem_pitch = pid.pitch.max as f32;
        } else if pid_i_mem_pitch < (pid.pitch.max as f32 * -1.0) {
            pid_i_mem_pitch = pid.pitch.max as f32 * -1.0;
        }

        pid_output_pitch = pid.pitch.p as f32 * pid_error_temp
            + pid_i_mem_pitch
            + pid.pitch.d as f32 * (pid_error_temp - pid_last_pitch_d_error);

        if pid_output_pitch > pid.pitch.max as f32 {
            pid_output_pitch = pid.pitch.max as f32;
        } else if pid_output_pitch < (pid.pitch.max as f32 * -1.0) {
            pid_output_pitch = pid.pitch.max as f32 * -1.0;
        }
        pid_last_pitch_d_error = pid_error_temp;

        //yaw calculation
        pid_error_temp = gyro_yaw_input - pid_yaw_setpoint;
        pid_i_mem_yaw += pid.yaw.i as f32 * pid_error_temp;
        if pid_i_mem_yaw > pid.yaw.max as f32 {
            pid_i_mem_yaw = pid.yaw.max as f32;
        } else if pid_i_mem_yaw < (pid.yaw.max as f32 * -1.0) {
            pid_i_mem_yaw = pid.yaw.max as f32 * -1.0;
        }
        pid_output_yaw = pid.yaw.p as f32 * pid_error_temp
            + pid_i_mem_yaw
            + pid.yaw.d as f32 * (pid_error_temp - pid_last_yaw_d_error);
        if pid_output_yaw > pid.yaw.max as f32 {
            pid_output_yaw = pid.yaw.max as f32;
        } else if pid_output_yaw < (pid.yaw.max as f32 * -1.0) {
            pid_output_yaw = pid.yaw.max as f32 * -1.0;
        }
        pid_last_yaw_d_error = pid_error_temp;
    }
}
