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

static mut gyro_x_calibration: i64 = 0;
static mut gyro_y_calibration: i64 = 0;
static mut gyro_z_calibration: i64 = 0;
static mut loop_of_calib: u16 = 0;

impl L3GD20H_Driver {
    pub fn new() -> L3GD20H_Driver {
        let settings = L3GD20GyroscopeSettings {
            DR: L3GD20GyroscopeDataRate::Hz760,
            BW: L3GD20GyroscopeBandwidth::BW4,
            power_mode: L3GD20PowerMode::Normal,
            zen: true,
            yen: true,
            xen: true,
            sensitivity: L3GD20GyroscopeFS::dps500,
            continuous_update: true,
            high_pass_filter_enabled: true,
            high_pass_filter_mode: Some(L3GD20GyroscopeHighPassFilterMode::NormalMode),
            high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_0),
        };

        let i2cdev = get_linux_l3gd20h_i2c_device("/dev/i2c-2".to_string()).unwrap();
        let l3gd20_gyro = L3GD20::new(i2cdev, settings).unwrap();
        return L3GD20H_Driver { gyro: l3gd20_gyro };
    }

    pub fn calibrate(&mut self) {
        unsafe {
            while (loop_of_calib < 2000) {
                let reading = self.gyro.read_gyroscope_raw().unwrap();
                gyro_x_calibration += reading.0 as i64;
                gyro_y_calibration += reading.1 as i64;
                gyro_z_calibration += reading.2 as i64;
                loop_of_calib += 1;
            }
            gyro_x_calibration /= 2000;
            gyro_y_calibration /= 2000;
            gyro_z_calibration /= 2000;

        }
    }

  /*  pub fn values(&mut self) -> data_angles {
        unsafe {
            if gyro_roll_calibration > 0.0 {
                let reading = self.gyro.read_gyroscope_raw().unwrap();
                return data_angles {
                    x: reading.0 as f32 - gyro_roll_calibration as f32,
                    y: reading.1 as f32 - gyro_pitch_calibration as f32,
                    z: reading.2 as f32 - gyro_yaw_calibration as f32,
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
    }*/
    pub fn raw_value(&mut self) -> raw_data {
        let data: (i16, i16, i16) = self.gyro.read_gyroscope_raw().unwrap();
        unsafe {
            if (loop_of_calib == 2000)
        {
            return raw_data {
                x: data.0 - gyro_x_calibration as i16,
                y: data.1 - gyro_y_calibration as i16,
                z: data.2 - gyro_z_calibration as i16,
            }
        }
        else {
            return raw_data {
                x: data.0,
                y: data.1,
                z: data.2,
            }
        }
        }
    }
}
