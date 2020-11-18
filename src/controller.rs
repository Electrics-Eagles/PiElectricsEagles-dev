const FREQ_CONST: i32 = 83; // Const to set freq of signalC
const PRESCALER: u8 = 83;

use linux_embedded_hal::{Delay, I2cdev};
use pwm_pca9685::{Address, Channel, Pca9685};

pub struct Controller {
    pwm: Pca9685<I2cdev>,
}
impl Controller {
    // initialize pmw i2c controller
   pub fn new(path_dev_i2c: String) -> Controller {
    let dev = I2cdev::new(path_dev_i2c).unwrap();
    let mut pwm_mod: Pca9685<I2cdev>  = Pca9685::new(dev, Address::default()).unwrap();
    pwm_mod.set_prescale(PRESCALER).unwrap(); // you need to set a correct prescaler 
    pwm_mod.set_channel_on_off(Channel::C0, 1000, 2000).unwrap();
    pwm_mod.set_channel_on_off(Channel::C1, 1000, 2000).unwrap();
    pwm_mod.set_channel_on_off(Channel::C2, 1000, 2000).unwrap();
    pwm_mod.set_channel_on_off(Channel::C3, 1000, 2000).unwrap();
    pwm_mod.enable().unwrap();
    Controller { pwm: pwm_mod }
   }
    // turn motor by selecting channel and speed 
   pub fn turn_motor(&mut self, addr_of_ch:Channel, value: u16)
   {
       self.pwm.set_channel_on(addr_of_ch, 0).unwrap();
   }
     // stop to rotate all motor 
   pub fn stop_all_motors(&mut self)
   {
    self.pwm.set_channel_full_off(Channel::C0).unwrap();
    self.pwm.set_channel_full_off(Channel::C1).unwrap();
    self.pwm.set_channel_full_off(Channel::C2).unwrap();
    self.pwm.set_channel_full_off(Channel::C3).unwrap();
   }
   pub fn set_throttle_external_pwm(&mut self, ch1:u16, ch2:u16, ch3:u16, ch4:u16) 
   {
       self.pwm.set_channel_on(Channel::C0, ch1).unwrap();
       self.pwm.set_channel_on(Channel::C1, ch2).unwrap();
       self.pwm.set_channel_on(Channel::C2, ch3).unwrap();
       self.pwm.set_channel_on(Channel::C3, ch4).unwrap();
   }
}
