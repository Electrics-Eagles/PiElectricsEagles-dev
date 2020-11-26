const FREQ_CONST: i32 = 83; // Const to set freq of signalC
const PRESCALER: u8 = 83;

use linux_embedded_hal::{Delay, I2cdev};
use pwm_pca9685::{Address, Channel, Pca9685};

pub struct Controller {
    pwm: Pca9685<I2cdev>,
}
impl Controller {
   pub fn new(path_dev_i2c: String) -> Controller {
    let dev = I2cdev::new(path_dev_i2c).unwrap();
    let mut pwm_mod: Pca9685<I2cdev>  = Pca9685::new(dev, Address::default()).unwrap();
    pwm_mod.set_prescale(PRESCALER).unwrap(); // you need to set a correct prescaler 
    pwm_mod.enable().unwrap();
    Controller { pwm: pwm_mod }
   }
   pub fn turn_motor(&mut self, addr_of_ch:Channel, value_micro: u16)
   {
       let mut value_pwm = value_micro * (4095 / 20000);
       self.pwm.set_channel_on(addr_of_ch, value_pwm).unwrap();
   }
   pub fn stop_all_motors(&mut self)
   {
    self.pwm.set_channel_full_off(Channel::C0).unwrap();
    self.pwm.set_channel_full_off(Channel::C1).unwrap();
    self.pwm.set_channel_full_off(Channel::C2).unwrap();
    self.pwm.set_channel_full_off(Channel::C3).unwrap();
   }
   pub fn set_throttle_external_pwm(&mut self, ch1:u16, ch2:u16, ch3:u16, ch4:u16) 
   {
       let mut value_pwm_ch1 = ch1 * (4095 / 20000);
       self.pwm.set_channel_on(Channel::C0, value_pwm_ch1).unwrap();
       
       let mut value_pwm_ch2 = ch2 * (4095 / 20000);
       self.pwm.set_channel_on(Channel::C1, value_pwm_ch2).unwrap();

       let mut value_pwm_ch3 = ch3 * (4095 / 20000);
       self.pwm.set_channel_on(Channel::C2, value_pwm_ch3).unwrap();

       let mut value_pwm_ch4 = ch4 * (4095 / 20000);  
       self.pwm.set_channel_on(Channel::C3, value_pwm_ch4).unwrap();
   }
}
