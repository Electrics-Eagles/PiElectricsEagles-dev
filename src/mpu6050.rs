#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use crate::config_parse::mpu_config_parser;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::*;
use mut_static::MutStatic;

pub struct MPU6050_Driver {
    value: Mpu6050<I2cdev, Delay>,
}

impl MPU6050_Driver {
    pub fn new(v: Mpu6050<I2cdev, Delay>) -> Self {
        MPU6050_Driver { value: v }
    }
    pub fn getvalue(&self) -> Mpu6050<I2cdev, Delay> {
        self.value
    }
    pub fn setvalue(&mut self, v: Mpu6050<I2cdev, Delay>) {
        self.value = v
    }
}

lazy_static! {
    static ref mpu_object: MutStatic<MPU6050_Driver> = MutStatic::new();
}

pub fn mpu6050_perpare() -> Mpu6050<I2cdev, Delay> {
    let mpu6050_conifg = mpu_config_parser();
    println!("{}", mpu6050_conifg.port);
    let i2c = I2cdev::new(mpu6050_conifg.port).expect("alert no port found");
    let delay = Delay;
    mpu_object
        .set(MPU6050_Driver::new(Mpu6050::new(i2c, delay)))
        .unwrap();
    mpu_object.read().unwrap().getvalue().init().unwrap();
    mpu_object
        .read()
        .unwrap()
        .getvalue()
        .soft_calib(Steps(mpu6050_conifg.sample_amount))
        .expect("software calibrate fallut");
    mpu_object
        .read()
        .unwrap()
        .getvalue()
        .calc_variance(Steps(mpu6050_conifg.sample_amount))
        .expect("calc variance error");
    return mpu_object.read().unwrap().getvalue();
}

pub struct GyroMpu6050RawData {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct AccMpu6050RawData {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub fn driver_mpu6050_version() -> &'static str {
    return " MPU6050 DRIVER  V0.0.1 verison is 14/11/2020 ID is: 4gQvYOdD";
}

pub fn get_acc_values(steps: u8) -> AccMpu6050RawData {
    let mut mpu = mpu_object.read().unwrap().getvalue();
    let data = AccMpu6050RawData {
        x: mpu.get_acc_avg(Steps(steps)).unwrap().x as u8,
        y: mpu.get_acc_avg(Steps(steps)).unwrap().y as u8,
        z: mpu.get_acc_avg(Steps(steps)).unwrap().z as u8,
    };
    return data;
}
pub fn get_gyro_values(steps: u8) -> GyroMpu6050RawData {
    let mut mpu = mpu_object.read().unwrap().getvalue();
    let data = GyroMpu6050RawData {
        x: mpu.get_gyro_avg(Steps(steps)).unwrap().x as i32,
        y: mpu.get_gyro_avg(Steps(steps)).unwrap().y as i32,
        z: mpu.get_gyro_avg(Steps(steps)).unwrap().z as i32,
    };
    return data;
}
pub fn get_temp() -> f32 {
    let mut mpu = mpu_object.read().unwrap().getvalue();
    return mpu.get_temp().unwrap();
}