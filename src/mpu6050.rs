use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};


pub fn mpu6050_perpare(port:String, step:u8) -> Mpu6050<I2cdev, Delay> {
    let i2c = I2cdev::new(port).expect("alert no port found");
    let delay = Delay;
    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init().unwrap();
    mpu.soft_calib(Steps(step)).expect("");
    mpu.calc_variance(Steps(step)).expect("");
    return mpu;
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
    return "MPU6050 DRIVER  V0.0.1 Beta 24/09/2020";
}

pub fn get_acc_values(mut mpu: Mpu6050<I2cdev, Delay>) -> acc_MPU6050_raw_data {
    let data  = acc_MPU6050_raw_data{
        x: mpu.get_acc().unwrap().x as i32,
        y: mpu.get_acc().unwrap().y as i32,
        z: mpu.get_acc().unwrap().z as i32
    };
    return data;
}
pub fn get_gyro_values(mut mpu: Mpu6050<I2cdev, Delay>) -> gyro_MPU650_raw_data {
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