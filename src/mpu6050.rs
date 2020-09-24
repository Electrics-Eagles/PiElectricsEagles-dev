use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

pub struct mpu60_perpare {
    
}
pub fn mpu6050_perpare() -> Mpu6050<Result<I2cdev, LinuxI2CError>, Delay> {
    let i2c = I2cdev::new("/dev/i2c-1");
    let mut delay=Delay;
    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;
    mpu.soft_calib(Steps(100))?;
    mpu.calc_variance(Steps(50))?;
    mpu
}
pub struct gyro_MPU650_raw_data {
    x:i32,
    y:i32,
    z:i32,
}

pub struct acc_MPU6050_raw_data {
    x:i32,
    y:i32,
    z:i32,
}


pub fn driver_mpu6050_version() -> &'static str {
    return "MPU6050 DRIVER  V0.0.1 Beta 21/09/2020";
}


pub fn get_acc_values(mut mpu: Mpu6050<I2cdev, Delay>) -> acc_MPU6050_raw_data {
    let data  = acc_MPU6050_raw_data{
        x: mpu.get_acc().unwrap().x as i32,
        y: mpu.get_acc().unwrap().y as i32,
        z: mpu.get_acc().unwrap().z as i32
    };
    return data;
}
pub fn get_gyro_values(mut mpu: Mpu6050<Result<I2cdev, LinuxI2CError>, Delay>) -> gyro_MPU650_raw_data {
    let data  = gyro_MPU650_raw_data{
        x: mpu.get_gyro().unwrap().x as i32,
        y: mpu.get_gyro().unwrap().y as i32,
        z: mpu.get_gyro().unwrap().z as i32
    };
    return data;

}
pub fn get_temp(mut mpu: Mpu6050<I2cdev, Delay>) -> f32 {
    return mpu.get_temp().unwrap()
}