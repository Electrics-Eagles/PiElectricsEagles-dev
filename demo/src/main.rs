
mod driver;
use crate::driver::set_throttle_external_pwm;

fn main() {
    println!("Hello, world!");

//external_pwm_prepare();
loop {
set_throttle_external_pwm(1500,1500,1500,1500);
}
}




/*
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address=Address::default();
//    println!("{}",address.Address);
    let mut pwm = Pca9685::new(dev, address).unwrap();
//pwm.enable_programmable_address();
//pwm.set_address(Address::from(0b1110000));
    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();
 pwm.enable();

loop {
    // Turn on channel 0 at 0.
for  x in 0..4095 { 
    pwm.set_channel_on(Channel::All,x ).unwrap();
}
    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
   // pwm.enable();
}

    let _dev = pwm.destr
*/
