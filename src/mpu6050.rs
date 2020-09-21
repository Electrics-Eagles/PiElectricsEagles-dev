use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use cgmath::prelude::*;
use cgmath::Vector3;
pub fn mpu6050_perpare() -> Mpu6050<Result<I2cdev, LinuxI2CError>, Delay> {
    let i2c = I2cdev::new("/dev/i2c-1");

    let mut delay=Delay;
    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;
    mpu.soft_calib(Steps(100))?;
    mpu.calc_variance(Steps(50))?;
    mpu
}



pub fn driver_mpu6050_version() -> &'static str {
    return "MPU6050 DRIVER  V0.0.1 Beta 21/09/2020";
}
pub fn get_gyro_values(mut mpu: Mpu6050<Result<I2cdev, LinuxI2CError>, Delay>) -> Owned<f32, U3, U1> {
    return mpu.get_gyro().unwrap().data;
}

pub fn get_acc_values(mut mpu: Mpu6050<I2cdev, Delay>) -> Owned<f32, U3, U1> {
    return mpu.get_acc().unwrap().data
}

pub fn get_temp(mut mpu: Mpu6050<I2cdev, Delay>) -> Vector3<f32> {
    return mpu.get_temp().unwrap().data
}