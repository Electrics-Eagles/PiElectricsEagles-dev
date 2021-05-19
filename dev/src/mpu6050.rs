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

const rad_s_to_deg_s: f32 = 180.0 / 3.14;
const g_to_raw: f32 = 4096.0;
const ms2_to_g:f32=1.0/9.81;


use crate::config_parse::config_parser;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::*;
use std::fs::File;
use std::io::prelude::*;
use core::time;
use std::thread;

/// Struct of raw data all axis
/// x: i32 - value of x-axis
/// y: i32 - value of y-axis
/// z: i32 - value of z-axis
pub struct GyroMpu6050RawData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// Struct of accelaration all axis
/// x: u8 - value of x-axis
/// y: u8 - value of y-axis
/// z: u8 - value of z-axis
pub struct AccMpu6050RawData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// It is a Mpu6050_driver object
/// mpu6050::Mpu6050_driver class (crate)
///
/// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
/// # Examples
///
/// ```
/// let mut mpu6050 = Mpu6050_driver::new(); // create object of Mpu6050_driver
/// let acc_value = mpu6050.get_acc_values(1); // get values of acceleration
/// let gyro_values = mpu6050.get_gyro_values(1); // get values of gyro
/// let mut temperature:f32 = mpu6050.get_temp(); // getting temperature of MPU6050 module
/// ```
///
pub struct Mpu6050_driver {
    value_of_gyro: Mpu6050<I2cdev, Delay>,
}
impl Mpu6050_driver {
    /// Returns Mpu6050_driver object
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// let mut mpu6050 = Mpu6050_driver::new();
    /// ```
    ///
    pub fn new() -> Mpu6050_driver {
        let delay_ = time::Duration::from_millis(500);
        let mut config = config_parser::new();
        let mpu6050_conifg = config.mpu_config_parser();
        let i2c = I2cdev::new(mpu6050_conifg.port).expect("alert no port found");
        let delay = Delay;
        let mut mpu = Mpu6050::new_with_sens(i2c, delay, AccelRange::G8, GyroRange::DEG500);
        mpu.wake().unwrap();
        thread::sleep(delay_);
        mpu.init().unwrap();
        thread::sleep(delay_);
        mpu.write_u8(0x1a, 0x06).unwrap();
        thread::sleep(delay_);
        mpu.soft_calib(Steps(200))
            .expect("software calibrate fallut");
        thread::sleep(delay_);
        mpu.calc_variance(Steps(200)).expect("calc variance error");
        Mpu6050_driver { value_of_gyro: mpu }
    }

    /// Show version driver for MPU6050+
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```&'static str```
    ///
    /// # Examples
    /// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
    ///
    ///
    pub fn driver_mpu6050_version() -> &'static str {
        return " MPU6050 DRIVER  V0.0.1 verison is 14/11/2020 ID is: 4gQvYOdD";
    }

    /// Get value of acceleartion in MPU6050
    /// # Arguments
    ///
    ///  steps
    ///
    /// # Return
    /// ```AccMpu6050RawData```
    ///
    /// # Examples
    /// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// let mut mpu6050 = Mpu6050_driver::new();
    /// let acc_value = mpu6050.get_acc_values(1);
    /// ```
    ///
    pub fn get_acc_values(&mut self) -> AccMpu6050RawData {
        let data = AccMpu6050RawData {
            x: (ms2_to_g*self.value_of_gyro.get_acc().unwrap().x * g_to_raw) as f64,
            y: (ms2_to_g*self.value_of_gyro.get_acc().unwrap().y * g_to_raw) as f64,
            z: (ms2_to_g*self.value_of_gyro.get_acc().unwrap().z * g_to_raw) as f64,
        };
        return data;
    }

    /// Get value of gyro-data in MPU6050
    /// # Arguments
    ///
    /// steps
    ///
    /// # Return
    /// ```GyroMpu6050RawData```
    ///
    /// # Examples
    /// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// let mut mpu6050 = Mpu6050_driver::new();
    /// let gyro_values = mpu6050.get_gyro_values(1);
    /// ```
    ///
    pub fn get_gyro_values(&mut self) -> GyroMpu6050RawData {
        let data = GyroMpu6050RawData {
            x: (self.value_of_gyro.get_gyro().unwrap().x * rad_s_to_deg_s) as f64,
            y: (self.value_of_gyro.get_gyro().unwrap().y * rad_s_to_deg_s) as f64,
            z: (self.value_of_gyro.get_gyro().unwrap().z * rad_s_to_deg_s) as f64,
        };
        return data;
    }

    /// Get value of temperature of sensor in MPU6050
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// Value of temperature in celusis degree
    /// ```f32```
    ///
    /// # Examples
    /// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// let mut mpu6050 = Mpu6050_driver::new();
    /// let mut temperature:f32 = mpu6050.get_temp();
    /// ```
    ///
    pub fn get_temp(&mut self) -> f32 {
        return self.value_of_gyro.get_temp().expect("error in fetch temp");
    }
}

