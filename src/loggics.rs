const START_MOTOS_VALUE:u16=10;





use std::error;

use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::Mpu6050;
use rppal::uart::Uart;

use crate::{controller::set_throttle_external_pwm, sbus::sbus_uart_init};
use crate::controller::external_pwm_prepare;
use crate::mpu6050::mpu6050_perpare;
use crate::sbus::read_sbus;

use pwm_pca9685::{Pca9685};

pub struct  ReadyHardware{
   mpu6050: Mpu6050<I2cdev, Delay>,
   motors_esc: Pca9685<I2cdev>,
   sbus:Uart,

}


pub fn start_motors(mut i2c_controller: Pca9685<I2cdev>) {
   set_throttle_external_pwm(i2c_controller,START_MOTOS_VALUE,START_MOTOS_VALUE,START_MOTOS_VALUE,START_MOTOS_VALUE);
}


pub fn init_hardware() -> ReadyHardware {
   let mpu6050= mpu6050_perpare();
   let sbus=sbus_uart_init();
   let motors_esc=external_pwm_prepare();
   let loaded_hardware=ReadyHardware{
      mpu6050:mpu6050,
      motors_esc:motors_esc,
      sbus: sbus,
     
  };
  return loaded_hardware
  

}
pub fn main_loop() {
    
 let hardware=init_hardware();
 let gyro=hardware.mpu6050;
 let reciver_values=hardware.sbus;
 let esc=hardware.motors_esc;
 let reciver=read_sbus(reciver_values);
   
   if reciver.ch1==1000 && reciver.ch3==1500 {
      start_motors(
         esc
      );

   }
   //make sbus


   


}