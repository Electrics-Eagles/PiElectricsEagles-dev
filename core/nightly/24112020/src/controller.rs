//config found on osciloscope
// min 1000 us value 205
// max 2000us value 410
// prescaller 120 -119
// diffeence 119

const PRESCALER: u8 = 120;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, time};
pub struct Controller {
    pwm: Pca9685<I2cdev>,
}

pub fn map(x: i64, in_min: i64, in_max: i64, out_min: i64, out_max: i64) -> i64 {
    let val = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    val
}

impl Controller {
    pub fn new() -> Controller {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let mut pwm_mod: Pca9685<I2cdev> = Pca9685::new(dev, Address::default()).unwrap();
        pwm_mod.set_prescale(PRESCALER).unwrap(); // you need to set a correct prescaler
        pwm_mod.set_channel_on(Channel::C0, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C1, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C2, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C3, 0).unwrap();
        pwm_mod.enable().unwrap();
       self. calibrate_esc ();
        Controller { pwm: pwm_mod }
    }
    pub fn set_throttle_external_pwm(&mut self, ch1: u16, ch2: u16, ch3: u16, ch4: u16) {
        let value_pwm_ch1 = map(ch1 as i64, 0, 20000, 0, 4095) as u16;
        println!("From :{}, OUT: {}", value_pwm_ch1, ch1);
        self.pwm
            .set_channel_on_off(Channel::C0, 0, value_pwm_ch1)
            .unwrap();

        let value_pwm_ch2 = map(ch2 as i64, 0, 20000, 0, 4095) as u16;
        self.pwm
            .set_channel_on_off(Channel::C1, 0, value_pwm_ch2)
            .unwrap();

        let value_pwm_ch3 = map(ch3 as i64, 0, 20000, 0, 4095) as u16;
        self.pwm
            .set_channel_on_off(Channel::C2, 0, value_pwm_ch3)
            .unwrap();

        let value_pwm_ch4 = map(ch4 as i64, 0, 20000, 0, 4095) as u16;
        self.pwm
            .set_channel_on_off(Channel::C3, 0, value_pwm_ch4)
            .unwrap();
        self.pwm.enable().unwrap();
    }
    pub fn test_esc(&mut self, pwm_duty: u16) {
        self.pwm
            .set_channel_on_off(Channel::C0, 0, pwm_duty)
            .unwrap();
        self.pwm.enable();
    }
    pub fn calibrate_esc(&mut self) {
        let value_pwm_max = map(2000, 0, 20000, 0, 4095) as u16;
        self.pwm
            .set_channel_on_off(Channel::C0, 0, value_pwm_max)
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C1, 0, value_pwm_max)
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C2, 0, value_pwm_max)
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C3, 0, value_pwm_max)
            .unwrap();
        thread::sleep(time::Duration::from_millis(3000));

        let value_pwm_min = map(1000, 0, 20000, 0, 4095) as u16;
        self.pwm
            .set_channel_on_off(Channel::C0, 0, value_pwm_min)
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C1, 0, value_pwm_min)
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C2, 0, value_pwm_min)
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C3, 0, value_pwm_min)
            .unwrap();
        thread::sleep(time::Duration::from_millis(3000));
    }
}
