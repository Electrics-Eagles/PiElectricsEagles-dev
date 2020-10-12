use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use crate::config_parse::mpu_config_parser;



pub fn mpu6050_perpare() -> Mpu6050<I2cdev, Delay> {
    let mut  inited=false;
    let mpu6050_conifg=mpu_config_parser();
    println!("{}",mpu6050_conifg.port);
    let i2c = I2cdev::new(mpu6050_conifg.port).expect("alert no port found");
    let delay = Delay;
    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init().unwrap();
    mpu.soft_calib(Steps(mpu6050_conifg.sample_amount)).expect("software calibrate fallut");
    mpu.calc_variance(Steps(mpu6050_conifg.sample_amount)).expect("calc variance error");
    return mpu;
}

pub struct GyroMpu6050RawData {
    pub x:i32,
    pub y:i32,
    pub z:i32,
}

pub struct AccMpu6050RawData {
    pub x:u8,
    pub y:u8,
    pub z:u8,
}

pub fn driver_mpu6050_version() -> &'static str {
    return "MPU6050 DRIVER  V0.0.1 Beta 29/09/2020";
}

pub fn get_acc_values(steps:u8) -> AccMpu6050RawData {
    let mut  inited=false;
    let mut  mpu=mpu6050_perpare();
    if(inited ==false )  { mpu=mpu6050_perpare();  inited=true;}
    else { println!("{}","ALREADY READY");}  
    let data  = AccMpu6050RawData{
        x: mpu.get_acc_avg(Steps(steps)).unwrap().x as u8,
        y: mpu.get_acc_avg(Steps(steps)).unwrap().y as u8,
        z: mpu.get_acc_avg(Steps(steps)).unwrap().z as u8,
    };
    return data;
}
pub fn get_gyro_values(steps:u8) -> GyroMpu6050RawData {
    let mut  inited=false;
    let mut  mpu;
    if(inited ==false && mpu==None)  { mpu=mpu6050_perpare();  inited=true;}
    else { println!("{}","ALREADY READY");}
    let data  = GyroMpu6050RawData{
        x: mpu.get_gyro_avg(Steps(steps)).unwrap().x as i32,
        y: mpu.get_gyro_avg(Steps(steps)).unwrap().y as i32,
        z: mpu.get_gyro_avg(Steps(steps)).unwrap().z as i32
    };
    return data;

}
pub fn get_temp() -> f32 {
    let mut  inited=false;
    let mut  mpu;
    if(inited ==false && mpu==None)  { mpu=mpu6050_perpare();  inited=true;}
    else { println!("{}","ALREADY READY");}
    return mpu.get_temp().unwrap()
}

