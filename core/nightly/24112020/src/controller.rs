//config found on osciloscope 
// min 1000 us value 205  
// max 2000us value 410 
// prescaller 120 -119 
// diffeence 119 


const PRESCALER: u8 = 120;

use crate::config_parse::mpu_config_parser;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
pub struct Controller {
    pwm: Pca9685<I2cdev>,
}

pub fn map(x: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    let val = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    val
}

impl Controller {
    pub fn new() -> Controller {
        let mpu6050_conifg = mpu_config_parser();
        let dev = I2cdev::new(mpu6050_conifg.port).unwrap();
        let mut pwm_mod: Pca9685<I2cdev> = Pca9685::new(dev, Address::default()).unwrap();
        pwm_mod.set_prescale(PRESCALER).unwrap(); // you need to set a correct prescaler
        pwm_mod.set_channel_on(Channel::C0, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C1, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C2, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C3, 0).unwrap();
        pwm_mod.enable().unwrap();
        Controller { pwm: pwm_mod }
    }
    /*
    pub fn turn_motor(&mut self, addr_of_ch: Channel, value_micro: u16) {
        let value_pwm = value_micro * (4095 / 20000);
        self.pwm.set_channel_on(addr_of_ch, value_pwm).unwrap();
    }
    pub fn stop_all_motors(&mut self) {
        self.pwm.set_channel_full_off(Channel::C0).unwrap();
        self.pwm.set_channel_full_off(Channel::C1).unwrap();
        self.pwm.set_channel_full_off(Channel::C2).unwrap();
        self.pwm.set_channel_full_off(Channel::C3).unwrap();
    }
    */
    /*
    pub fn set_throttle_external_pwm(&mut self, ch1: u16, ch2: u16, ch3: u16, ch4: u16) {
        let mut value_pwm_ch1 = ch1 * (4095 / 20000);
        self.pwm.set_channel_on(Channel::C0, value_pwm_ch1).unwrap();

        let mut value_pwm_ch2 = ch2 * (4095 / 20000);
        self.pwm.set_channel_on(Channel::C1, value_pwm_ch2).unwrap();

        let mut value_pwm_ch3 = ch3 * (4095 / 20000);
        self.pwm.set_channel_on(Channel::C2, value_pwm_ch3).unwrap();

        let mut value_pwm_ch4 = ch4 * (4095 / 20000);
        self.pwm.set_channel_on(Channel::C3, value_pwm_ch4).unwrap();
    }
    */

    pub fn set_throttle_external_pwm(&mut self, ch1: u16, ch2: u16, ch3: u16, ch4: u16) {
        let value_pwm_ch1 = map(ch1 as i32, 205, 410, 0, 4095) as u16;
        self.pwm.set_channel_on(Channel::C0, value_pwm_ch1).unwrap();

        let value_pwm_ch2 = map(ch2 as i32, 205, 410, 0, 4095) as u16;
        self.pwm.set_channel_on(Channel::C1, value_pwm_ch2).unwrap();

        let value_pwm_ch3 = map(ch3 as i32, 205, 410, 0, 4095) as u16;
        self.pwm.set_channel_on(Channel::C2, value_pwm_ch3).unwrap();

        let value_pwm_ch4 = map(ch4 as i32, 205, 410, 0, 4095) as u16;
        self.pwm.set_channel_on(Channel::C3, value_pwm_ch4).unwrap();
    }
}
