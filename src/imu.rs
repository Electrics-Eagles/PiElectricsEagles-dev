
use mpu6050::{Mpu6050Error, Mpu6050};
use linux_embedded_hal::{I2cdev, Delay};
use mpu6050::device::{AccelRange, GyroRange, ACCEL_HPF, GYRO_REGX_H, ACC_REGX_H};
use crate::config_parse::config_parser;
use crate::utils;

pub struct  imu {
    imu: Mpu6050<I2cdev>,
}

pub struct ImuData {
    pub roll:i32,
    pub pitch:i32,
    pub yaw:i32,

}

static mut gyro_roll_calibration: i32 = 0;
static mut gyro_pitch_calibration: i32 = 0;
static mut gyro_yaw_calibration: i32 = 0;
static mut loop_of_calib: u16 = 0;
impl imu {

    pub fn new() -> Self {
        let  mut config = config_parser::new();
        let i2c = I2cdev::new(config.imu_config_parser().port)
            .map_err(Mpu6050Error::I2c).unwrap();
        let mut delay = Delay;
        let mut mpu = Mpu6050::new_with_sens(i2c, AccelRange::G8, GyroRange::D500);
        mpu.init(&mut delay).unwrap();
        println!("*---------------------------Custom MPU6050 config--------------------------------*");
        utils::delay(500);
        mpu.write_byte(0x1A ,0x04).expect("MPU Config fatal");
        utils::delay(1000);
        mpu.write_byte(0x1B ,0x04).expect("MPU Config fatal");
        utils::delay(1000);
        println!("*---------------------------Custom MPU6050 config--------------------------------*");
        println!("*---------------------------ACCEL_HPF set to 5Hz--------------------------------*");
        mpu.set_accel_hpf(ACCEL_HPF::_5).expect("MPU Config fatal");
        utils::delay(1000);
        println!("*---------------------------ACCEL_HPF set to 5Hz--------------------------------*");
        imu { imu: mpu }
    }

    pub fn get_acc_data(&mut self) -> ImuData {
        let  acc = self.imu.read_rot(ACC_REGX_H).unwrap();
        let data=ImuData{
            roll: acc.x as i32,
            pitch: acc.y as i32,
            yaw: acc.z as i32
        };

        data
    }
    pub fn calibrate(&mut self) {
        unsafe {
            while loop_of_calib < 2000 {
                let reading = self.get_gyro_data();
                gyro_roll_calibration += reading.pitch;
                gyro_pitch_calibration += reading.roll;
                gyro_yaw_calibration += reading.yaw ;
                loop_of_calib += 1;
            }
            gyro_roll_calibration /= 2000;
            gyro_pitch_calibration /= 2000;
            gyro_yaw_calibration /= 2000;
        }
    }
    pub fn get_normalised_gyro_data(&mut self) -> ImuData {
        let data = self.get_gyro_data();
        unsafe {
            if loop_of_calib == 2000
            {
                return ImuData {
                    roll: data.roll - gyro_roll_calibration,
                    pitch: data.pitch - gyro_pitch_calibration,
                    yaw: data.yaw - gyro_yaw_calibration,
                }
            }
            else {
                panic!("No calibration . That is nightmare");
            }
        }
    }

    fn swap(order:String,x:u8,y:u8,z:u8)  -> (u8, u8, u8)  {
        let data: (u8, u8, u8) = match order.as_str() {
            "xzy" => (x, z, y),
            "xzy" => (x, z, y),
            "yxz" =>  (y, x, z),
            "yzx" => (y, z, x),
            "zxy" => (z, x, y),
            "zyx" => (z, y, x),
            _ => (0, 0, 0),
        };

     return data
    }

    fn get_gyro_data(&mut self) -> ImuData {
        let  gyro = self.imu.read_rot(GYRO_REGX_H).unwrap();

        let data=ImuData{
            roll: gyro.x as i32,
            pitch: gyro.y as i32,
            yaw: gyro.z as i32
        };

        data

    }

}
