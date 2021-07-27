//
//
// Alex Zaslavkis (Electrics Eagles) 2021
//
// ---------------- MPU6050 Driver --------------------
//
// Simple software level that retrive data from MPU6050 sensor. Works with the main code file
//
// typical usage :
//
// 	new(); // create struct of Mpu6050_driver
//  get_acc_values(&mut self, steps: u8); // get data of acceleration all axis from MPU6050
//  get_gyro_values(&mut self, steps: u8); // get data of gyro all axis from MPU6050
//  get_temp(&mut self); // get data of temperature in MPU6050
//  driver_mpu6050_version(); // Show version driver for MPU6050
//
//
// Enjoy

extern crate i2cdev_l3gd20;
extern crate i2csensors;

use i2cdev_l3gd20::*;
use i2csensors::{Gyroscope, Vec3};

pub struct data_angles {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct raw_data {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}


pub struct L3GD20H_Driver {
    gyro: L3GD20<i2cdev::linux::LinuxI2CDevice>,
}

static mut gyro_roll_calibration: f64 = 0.0;
static mut gyro_pitch_calibration: f64 = 0.0;
static mut gyro_yaw_calibration: f64 = 0.0;

impl L3GD20H_Driver {
    pub fn new() -> L3GD20H_Driver {
        let settings = L3GD20GyroscopeSettings {
            DR: L3GD20GyroscopeDataRate::Hz95,
            BW: L3GD20GyroscopeBandwidth::BW1,
            power_mode: L3GD20PowerMode::Normal,
            zen: true,
            yen: true,
            xen: true,
            sensitivity: L3GD20GyroscopeFS::dps500,
            continuous_update: true,
            high_pass_filter_enabled: true,
            high_pass_filter_mode: Some(L3GD20GyroscopeHighPassFilterMode::NormalMode),
            high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_9),
        };

        let mut i2cdev = get_linux_l3gd20h_i2c_device("/dev/i2c-2".to_string()).unwrap();
        let mut l3gd20_gyro = L3GD20::new(i2cdev, settings).unwrap();
        return L3GD20H_Driver { gyro: l3gd20_gyro };
    }

    pub fn calibrate(&mut self) {
        unsafe {
            for a in 0..2000 {
                let reading = self.gyro.angular_rate_reading().unwrap();
                gyro_roll_calibration += reading.x as f64;
                gyro_pitch_calibration += reading.y as f64;
                gyro_yaw_calibration += reading.z as f64;
            }
            gyro_roll_calibration /= 2000.0;
            gyro_pitch_calibration /= 2000.0;
            gyro_yaw_calibration /= 2000.0;
        }
    }

    pub fn values(&mut self) -> data_angles {
        unsafe {
            if gyro_roll_calibration > 0.0 {
                let reading = self.gyro.angular_rate_reading().unwrap();
                return data_angles {
                    x: reading.x - gyro_roll_calibration as f32,
                    y: reading.y - gyro_pitch_calibration as f32,
                    z: reading.z - gyro_yaw_calibration as f32,
                };
            } else {
                println!("{}", "did you calibaretd gyro??");
                data_angles {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        }
    }
    pub fn raw_value(&mut self) -> raw_data {
        let mut data: (i16, i16, i16) = self.gyro.read_gyroscope_raw().unwrap();
        return raw_data {
            x: data.0,
            y: data.1,
            z: data.2,
        };
    }
}
