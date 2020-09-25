
use crate::sbus::sbus_uart_init;
use crate::controller::external_pwm_prepare;
use crate::mpu6050::mpu6050_perpare;


pub fn init_harware() {
    
   let mpu6050= mpu6050_perpare();
   let sbus=sbus_uart_init();
   let motors_esc=external_pwm_prepare();
   


   


}