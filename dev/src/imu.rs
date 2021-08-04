
use mpu6050::{Mpu6050Error, Mpu6050};
use linux_embedded_hal::{I2cdev, Delay};
use mpu6050::device::{AccelRange, GyroRange, ACCEL_HPF, GYRO_REGX_H, ACC_REGX_H};
use crate::config_parse::config_parser;
/// IMU unit struct
/// so there are object of imu that has methods and functions .
/// So the usage inside imu.rs is :
/// ```self.imu.method();
/// ```
/// So the usage outside :
/// ```
/// let mut imu =imu::imu::new(portname);
/// let gyro= imu.method();
/// ```
///
///
pub struct  imu {
     imu: Mpu6050<I2cdev>,
}
///ImuData that has i32 struct and has 3 axis
/// roll or (x)
/// pitch or (y)
/// yaw or (z)
///
/// Also it is used in all functions that are as struct .
/// ImuData is universal way to get pitch/yaw/roll in code
pub struct ImuData {
    pub roll:i32,
    pub pitch:i32,
    pub yaw:i32,

}


/// The calibration gyro_roll_calibration value where is stored the roll calibration value
/// The calibration gyro_pitch_calibration value where is stored the pitch  calibration value
/// The calibration gyro_yaw_calibration value where is stored the yaw calibration value
///Value that counts loops of calibration loop_of_calib


static mut gyro_roll_calibration: i32 = 0;
static mut gyro_pitch_calibration: i32 = 0;
static mut gyro_yaw_calibration: i32 = 0;
static mut loop_of_calib: u16 = 0;
/// the implemantion imu
impl imu {
    /// the new imu object  creating new functions .
    /// Usage:
    /// ```
    /// use crate::imu;
    /// let mut imu =imu::imu::new(port);
    /// let acc = imu.get_acc_data();
    /// let gyro= imu.get_normalised_gyro_data();
    ///
    /// ```
    /// As argument in requires the path of i2c (String)
    pub fn new() -> Self {
        let  mut config = config_parser::new();
        let i2c = I2cdev::new(config.imu_config_parser().port)
            .map_err(Mpu6050Error::I2c).unwrap();
        let mut delay = Delay;
        let mut mpu = Mpu6050::new_with_sens(i2c, AccelRange::G8, GyroRange::D500);
        mpu.init(&mut delay).unwrap();
        mpu.set_accel_hpf(ACCEL_HPF::_5);
        mpu.set_sleep_enabled(false);
        imu { imu: mpu }
    }
     /// The get_acc_data() is function that return raw values acc the returns ImuData struct .
     /// Usage is :
     /// ```
     /// let data=imu.get_acc_data();
     /// ```
     ///
     pub fn get_acc_data(&mut self) -> ImuData {
        let  acc = self.imu.read_rot(ACC_REGX_H).unwrap();
        let data=ImuData{
            roll: acc.x as i32,
            pitch: acc.y as i32,
            yaw: acc.z as i32
        };

        data
    }
/// Calibrate function .
/// Is a funcion that calibrate only gyro before flight because the sensor can be tilted and to avoid it this function is used.
/// Usage:
/// ```
/// imu.calibrate();
/// ```
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

    /// Function that returns raw data - calibrate value
    /// !!!WARRING : it use unsafe blocks !!!
    /// As result is struct ImuData.
    /// Usage:
    /// ```
    /// let data=imu.get_normalised_gyro_data();
    /// ```
    ///
    /// Also can cause panic if gyro is not calibrated  . Be careful
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

    /// The get_gyro_data() is function that return raw values gyro the returns ImuData struct .
    /// Usage is :
    /// ```
    /// let data=imu.get_gyro_data();
    /// ```
    ///

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

