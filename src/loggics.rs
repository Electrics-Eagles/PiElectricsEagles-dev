const START_MOTOS_VALUE:u16=10;
const STEP:u8=10;





use std::error;
use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::Mpu6050;
use rppal::uart::Uart;
extern crate pid;
use pid::Pid;

use crate::{controller::set_throttle_external_pwm, sbus::sbus_uart_init};
use crate::controller::external_pwm_prepare;
use crate::mpu6050::mpu6050_perpare;
use crate::sbus::read_sbus;
use crate::mpu6050::get_acc_values;
use crate::config_parse::AutoLevel_Config;

use pwm_pca9685::{Pca9685};

pub struct  ReadyHardware{
   mpu6050: Mpu6050<I2cdev, Delay>,
   motors_esc: Pca9685<I2cdev>,
   sbus:Uart,

}


pub fn start_motors(mut i2c_controller: Pca9685<I2cdev>) {
   set_throttle_external_pwm(i2c_controller,START_MOTOS_VALUE,START_MOTOS_VALUE,START_MOTOS_VALUE,START_MOTOS_VALUE);
}

fn convert(v:u8) -> f64 {
   return v as f64;
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
pub fn calc_pid(){
   
pub fn main_loop() {
 
 let mut loops=0;
 let hardware=init_hardware();
 let mpu6050= mpu6050_perpare();
 
 let reciver_values=hardware.sbus;
 let esc=hardware.motors_esc;
 let reciver=read_sbus(reciver_values);

 let autolevel=AutoLevel_Config();




   let acc_value=get_acc_values(mpu6050,1);
   let acc_x=acc_value.x;
   let acc_y=acc_value.y;
   let acc_z=acc_value.z;
   let acc_total_vector_no_square = (acc_x.pow(2)+acc_y.pow(2)+acc_z.pow(2)) as f64;
   let acc_total_vector:f64=acc_total_vector_no_square.sqrt();
   let mut angle_pitch_acc:f64=0.0;
   let mut angle_roll_acc:f64=0.0;
   let mut angle_pitch:f64=0.0;
   let mut angle_roll:f64=0.0;
   let mut pitch_level_correction:f64=0.0;
   let mut roll_level_correction:f64=0.0;
   

 
   angle_pitch += convert(acc_x)* 0.0000611;                                    //Calculate the traveled pitch angle and add this to the angle_pitch variable.
   angle_roll += convert(acc_z)* 0.0000611;

   
   if convert(acc_y).abs() < acc_total_vector {
      angle_pitch_acc = (convert(acc_y)/acc_total_vector).asin()* 57.296;  
   }
   if convert(acc_x).abs() < acc_total_vector {
      angle_roll_acc = (convert(acc_x)/acc_total_vector).asin()* -57.296;  
   }
   angle_pitch_acc -= 0.0;                                                   
   angle_roll_acc -= 0.0;  

   angle_pitch = angle_pitch * 0.9996 + angle_pitch_acc * 0.0004;            //Correct the drift of the gyro pitch angle with the accelerometer pitch angle.
   angle_roll = angle_roll * 0.9996 + angle_roll_acc * 0.0004;               //Correct the drift of the gyro roll angle with the accelerometer roll angle.

   pitch_level_correction = angle_pitch * 15 as f64;                                    //Calculate the pitch angle correction
   roll_level_correction = angle_roll * 15 as f64;                                      //Calculate the roll angle correction
    


  if autolevel==0{                                                          //If the quadcopter is not in auto-level mode
   pitch_level_correction = 0.0;                                                 //Set the pitch angle correction to zero.
   roll_level_correction = 0.0;                                                  //Set the roll angle correcion to zero.
 }
 

   
  loops=loops+1;


  if reciver.ch1==1000 && reciver.ch3==1500 {
   start_motors(
      esc
   );
 

//while(receiver_input_channel_3 < 990 || receiver_input_channel_3 > 1020 || receiver_input_channel_4 < 1400){
   
 angle_pitch = angle_pitch_acc;                                          //Set the gyro pitch angle equal to the accelerometer pitch angle when the quadcopter is started.
 angle_roll = angle_roll_acc;  
                                           //Set the gyro roll angle equal to the accelerometer roll angle when the quadcopter is started.
 
   let mut pid = Pid::new(10.0, 0.0, 0.0, 100.0, 100.0, 100.0, 15.0);
   let output = pid.next_control_output(10.0);
   
}
}

   //make sbus


   


}