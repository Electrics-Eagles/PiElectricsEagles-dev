/*
 *
 * For check UART /IBUS /PWM GPIO / ADC
 *
 *
 * UART - open uart using serial +
 * IBUS using our lib try parse data +
 * ADC using lib get voltage should not  be zero
 * Buzzer rppal . make noise ^) +
 * LED test blink +
 * I@C PWM test and gyro +
 */
use crate::batterycontroller::*;
use crate::controller::Controller;
use crate::imu::imu;
use crate::reciver;
use crate::reciver::*;
use crate::utils::delay;
use rppal::gpio::Gpio;
use std::{thread, time};

pub fn test() {
    let mut reciver_driver = receiver::new();
    let ibus_throllite = reciver_driver.get_datas_of_channel_form_ibus_receiver()[2];

    println!("{}", "Set reciver throlite to max value");
    delay(5000);
    if ibus_throllite == 2000 {
        println!("Component 1 = pass");
    } else {
        println!("Component 1 = fail");
        panic!("Component 1 = fail");
    }
    let mut imu = imu::new();
    imu.calibrate();
    println!("It you see this Component 2 = pass");
    let mut controller = Controller::new();
    controller.set_throttle_external_pwm(1300, 1300, 1300, 1300);
    println!("It you see this Component 3 = pass");
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(7).unwrap().into_output();
    pin.set_high();
    delay(5000);
    println!("It you hear  sound  Component 4 = pass");

    let mut led_one = gpio.get(8).unwrap().into_output();
    let mut led_two = gpio.get(17).unwrap().into_output();
    delay(5000);
    for _ in 0..10 {
        led_one.set_high();
        led_two.set_high();
        delay(1000);
        led_one.set_low();
        led_two.set_low();
    }
    println!("It you see  blinking led  Component 5,6 = pass");
    let mut battery = BatteryController::init();
    let voltage_lipo_battery = battery.measure_voltage_of_lipo_battery(); // Measure common voltage of lipo battery
    println!("Main battery voltage: {:.2} V", voltage_lipo_battery); // Print voltage in console (OPTIONAL)
    if voltage_lipo_battery > 0.0 {
        println!("It you see  blinking led  Component 7 = pass");
    }

    println!("Well done all working..");
}
