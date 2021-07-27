
//
//
// Misha Zaslavskis (Electrics Eagles) 2021
//
// ---------------- LIS3DH Driver --------------------
//
// Simple software level that retrive data from LIS3DH sensor. Works with the main code file
//
// typical usage :
//
// 	new(); // create struct of LIS3DH_Driver
//  init(); // initialize acclerometer
//  get_data_raw(); // retrive raw data from accelerometer measurement. Each axis are set value as i16
//  get_data_g(); // retrive g-value data from accelerometer measuremt. Each axis are set value as f32
//
// Enjoy
use lis3dh::accelerometer::{RawAccelerometer, Accelerometer};
use linux_embedded_hal::I2cdev;

pub struct data_acc_angles {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct raw_acc_data {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct LIS3DH_Driver {
    acc: lis3dh::Lis3dh<linux_embedded_hal::I2cdev>
}

impl LIS3DH_Driver {
    pub fn new() -> Self
    {
        let  dev = I2cdev::new("/dev/i2c-3").unwrap();
        LIS3DH_Driver { acc: lis3dh::Lis3dh::new(dev, lis3dh::SlaveAddr::Default).expect("some error") }
    }
    pub fn init(&mut self)
    {
        self.acc.set_mode(lis3dh::Mode::HighResolution).unwrap();
        self.acc.set_datarate(lis3dh::DataRate::Hz_200).unwrap();
        self.acc.set_range(lis3dh::Range::G8).unwrap();
    }
    pub fn get_data_raw(&mut self) -> raw_acc_data
    {
        let accel_r = self.acc.accel_raw().unwrap(); // get raw -value (i16 each axis)
        raw_acc_data { x: accel_r.x, y: accel_r.y, z: accel_r.z}
    }

}
