const FREQ_CONST: i32 = 200; // Const to set freq of signal

use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

fn check_driver_config_check(driver: String, amount: u8) {
    if !amount == 4 || !driver.eq("pca9685") {
        panic!("Incorrect module in config");
    }
}

pub fn external_pwm_prepare(port: String, amount: u8, driver: String) -> Pca9685<I2cdev> {
    check_driver_config_check(driver, amount);
    let dev = I2cdev::new(port).unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();
    pwm.set_prescale(FREQ_CONST as u8).unwrap();
    return pwm;
}

pub fn get_esc_verison() -> &'static str {
    return "ESC PWM  MODULE VERSION 0.0.1 18/09/2020";
}

pub fn set_throttle_external_pwm(mut i2c_controller: Pca9685<I2cdev>, ch1: u16, ch2: u16, ch3: u16, ch4: u16) {
    /*
    It is 12-bit PWM be careful 4095
     */
    i2c_controller.set_channel_on(Channel::C0, ch1).unwrap();
    i2c_controller.set_channel_on(Channel::C1, ch2).unwrap();
    i2c_controller.set_channel_on(Channel::C2, ch3).unwrap();
    i2c_controller.set_channel_on(Channel::C3, ch4).unwrap();
}

