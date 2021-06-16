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

use self::i2cdev_l3gd20::{L3GD20GyroscopeSettings, L3GD20GyroscopeDataRate, L3GD20GyroscopeBandwidth, L3GD20PowerMode, L3GD20GyroscopeFS, L3GD20GyroscopeHighPassFilterMode, L3GD20HighPassFilterCutOffConfig, get_linux_l3gd20_i2c_device, L3GD20, get_linux_l3gd20h_i2c_device};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use self::i2csensors::Gyroscope;

pub struct data_angles {
    pub x:f32,
    pub y:f32,
    pub z:f32
}


pub struct L3GD20H_Driver{
    gyro:L3GD20<i2cdev::linux::LinuxI2CDevice>,
}
impl L3GD20H_Driver {
    pub fn new() -> L3GD20H_Driver {
        let settings = L3GD20GyroscopeSettings {
            DR: L3GD20GyroscopeDataRate::Hz95,
            BW: L3GD20GyroscopeBandwidth::BW4,
            power_mode: L3GD20PowerMode::Normal,
            zen: true,
            yen: true,
            xen: true,
            sensitivity: L3GD20GyroscopeFS::dps500,
            continuous_update: true,
            high_pass_filter_enabled: true,
            high_pass_filter_mode: Some(L3GD20GyroscopeHighPassFilterMode::NormalMode),
            high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_0)
        };


        let mut i2cdev = get_linux_l3gd20h_i2c_device().unwrap();

        let mut l3gd20_gyro = L3GD20::new(i2cdev, settings).unwrap();


        return L3GD20H_Driver { gyro: l3gd20_gyro };
    }

        pub fn values(&mut self) -> data_angles {
            let reading = self.gyro.angular_rate_reading().unwrap();
            return data_angles {
                x: reading.x,
                y: reading.y,
                z: reading.z,
            }
        }
    }
