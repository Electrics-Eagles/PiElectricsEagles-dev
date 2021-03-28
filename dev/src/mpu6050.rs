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
const rad_s_to_deg_s:f32=180.0/3.14;
const g_to_raw:f32=4096.0;
use crate::config_parse::config_parser;
use crate::simple_logger;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::*;
use std::fs::File;
use std::io::prelude::*;
use simple_logger::*;

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
        let mut config = config_parser::new();
        let mpu6050_conifg = config.mpu_config_parser();
        simple_logger::write_log(LevelOfLog::INFO,"READ MPU Config".parse().unwrap());
        let i2c = I2cdev::new(mpu6050_conifg.port).expect("alert no port found");
        let delay = Delay;
        let mut mpu = Mpu6050::new_with_sens(i2c, delay,AccelRange::G8,GyroRange::DEG500);
        mpu.init().unwrap();
        mpu.soft_calib(Steps(200))
            .expect("software calibrate fallut");
        mpu.calc_variance(Steps(200))
            .expect("calc variance error");


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
    pub fn get_acc_values(&mut self, steps: u8) -> AccMpu6050RawData {
        simple_logger::write_log(LevelOfLog::INFO,"Read acc values".parse().unwrap());
        let data = AccMpu6050RawData {
            x: (self.value_of_gyro.get_acc().unwrap().x * rad_s_to_deg_s) as f64,
            y: (self.value_of_gyro.get_acc().unwrap().y * rad_s_to_deg_s) as f64,
            z: (self.value_of_gyro.get_acc().unwrap().z * rad_s_to_deg_s) as f64,
        };
        simple_logger::write_log(LevelOfLog::INFO, "ACC VALUE:".parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, data.x.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, data.y.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, data.z.to_string().parse().unwrap());
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
    pub fn get_gyro_values(&mut self, steps: u8) -> GyroMpu6050RawData {
        simple_logger::write_log(LevelOfLog::INFO, "Read gyro values".parse().unwrap());
        let data = GyroMpu6050RawData {
            x: (self.value_of_gyro.get_gyro().unwrap().x * g_to_raw) as f64,
            y: (self.value_of_gyro.get_gyro().unwrap().y * g_to_raw) as f64,
            z: (self.value_of_gyro.get_gyro().unwrap().z * g_to_raw) as f64,
        };
        simple_logger::write_log(LevelOfLog::INFO, "GYRO VALUE:".parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, data.x.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, data.y.to_string().parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, data.z.to_string().parse().unwrap());
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
        simple_logger::write_log(LevelOfLog::INFO, "Read temp values".parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO, "GYRO VALUE:".parse().unwrap());
        simple_logger::write_log(LevelOfLog::INFO,
            self.value_of_gyro
                .get_temp()
                .expect("error in fetch temp")
                .to_string()
                .parse()
                .unwrap(),
        );
        return self.value_of_gyro.get_temp().expect("error in fetch temp");
    }
}
