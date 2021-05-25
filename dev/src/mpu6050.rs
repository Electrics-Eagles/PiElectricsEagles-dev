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

const RAD_S_TO_DEG_S: f32 =57.2958;
const RAD_TO_DEG:f32=57.2958;
const G_TO_RAW:f32=9.8066;

use crate::config_parse::config_parser;
use core::time;
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

use std::fs::File;
use std::io::prelude::*;
use std::thread;
use mpu6050::device::AccelRange;
use mpu6050::device::ACCEL_HPF::_1P25;
use mpu6050::device::GyroRange;

/// Struct of raw data all axis
/// x: i32 - value of x-axis
/// y: i32 - value of y-axis
/// z: i32 - value of z-axis
pub struct GyroMpu6050Data {
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

pub struct AccMpu6050Angles{
    pub roll:f64, //x
    pub pitch:f64, //y
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
    value_of_gyro: Mpu6050<I2cdev>,
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
        let i2c = I2cdev::new(mpu6050_conifg.port)
            .map_err(Mpu6050Error::I2c).unwrap();
        let mut delay = Delay;
        let mut mpu = Mpu6050::new_with_sens(i2c, AccelRange::G8, GyroRange::D500);
        mpu.init(&mut delay).unwrap();
        thread::sleep(delay_);
        mpu.set_accel_hpf(_1P25).unwrap();
        thread::sleep(delay_);
        mpu.write_byte(0x1a, 0x06).unwrap();
        thread::sleep(delay_);

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
        let input_data = self.value_of_gyro.get_acc().unwrap();
        let data = AccMpu6050RawData {
            x: (input_data.x * G_TO_RAW) as f64,
            y: (input_data.y * G_TO_RAW) as f64,
            z: (input_data.z * G_TO_RAW) as f64,
        };
        return data
    }

    /// Get value of gyro-data in MPU6050
    /// # Arguments
    ///
    /// steps
    ///
    /// # Return
    /// ```GyroMpu6050Data```
    ///
    /// # Examples
    /// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// let mut mpu6050 = Mpu6050_driver::new();
    /// let gyro_values = mpu6050.get_gyro_values(1);
    /// ```
    ///
    pub fn get_gyro_values(&mut self) -> GyroMpu6050Data {
        let input_data = self.value_of_gyro.get_gyro().unwrap();
        let data = GyroMpu6050Data {
            x: (input_data.x * RAD_S_TO_DEG_S) as f64,
            y: (input_data.x * RAD_S_TO_DEG_S) as f64,
            z: (input_data.x * RAD_S_TO_DEG_S) as f64,
        };
        return data
    }

    pub fn get_acc_angles(&mut self)  -> AccMpu6050Angles{
        let input_data = self.value_of_gyro.get_acc_angles().unwrap();

        let data = AccMpu6050Angles {
            roll: (input_data.x * RAD_TO_DEG) as f64,
            pitch: (input_data.y * RAD_TO_DEG) as f64,
        };
        return data
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
    ///
    ///
    ///
    pub fn get_temp(&mut self) -> f32 {
        return self.value_of_gyro.get_temp().expect("error in fetch temp");
    }
}
