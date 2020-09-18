const FREQ_CONST:i32  = 200;

use linux_embedded_hal::I2cdev;

use pwm_pca9685::{Address, Channel, Pca9685};
fn check(driver:String,amount:u8){
    let mut correct =1;
    if amount==4 {
        correct=correct+1;
    }
    else {
        correct=correct;
    }
    if driver.eq("pca9685") {
        correct=correct+1;
    }
    else {
        correct=correct;
    }
    if correct < 2 {
        panic!("Incorrect module in config");
    }

}

pub fn prepare(port:String,amount:u8,driver:String) -> Pca9685<I2cdev> {
    check(driver,amount);
    let dev = I2cdev::new(port).unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();
    pwm.set_prescale(FREQ_CONST as u8).unwrap();
    return pwm;
}
pub fn esc_verison() -> &'static str {
    return "ESC PWM  MODULE VERSION 0.0.1 18/09/2020";
}
/*
fn main() {

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    // Turn on channel 0 at 0.
    pwm.set_channel_on(Channel::C0, 0).unwrap();

    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    pwm.set_channel_off(Channel::C0, 2047).unwrap();

    let _dev = pwm.destroy(); // Get the I2C device back
}
*/
