//
//
// Alex Zaslavkis (Electrics Eagles) 2021
//
// ---------------- Controller Driver --------------------
//
// Simple software level that esc launch for drone.
//
//
// typical usage :
//    new(); // create struct of esc driver
//    calibrate_esc(); // auto-calibrate for esc motor before launch. This function is mandatory to use before launching esc.
//    set_throttle_external_pwm(ch1: u16, ch2: u16, ch3: u16, ch4: u16); // set throttle to how speed run each esc with selectable value 1000-2000
//    test_esc(pwm_duty: u16); // check esc how run each motor with value of pwm duty same
//
//    let mut controller = Controller::new(); // Initialize Contriller flieds
//    controller.calibrate_esc(); // Calibrate esc
//    controller.set_throttle_external_pwm(1050, 1050, 1050, 1050, 1050); // Launch esc
//
//    config found on osciloscope
//    min 1000 us value 205
//    max 2000us value 410
//    prescaller 120 -119
//    diffeence 119
//
// Enjoy

/// Clock prescaler of MCU raspberry pi zero
const PRESCALER: u8 = 120;

use crate::config_parse::config_parser;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, time};

pub fn map(x: i64, in_min: i64, in_max: i64, out_min: i64, out_max: i64) -> i64 {
    let val = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    val
}

/// It is a Controller object
/// Controller class (crate)

///
/// Made by : Misha Zaslavkis (Electrics Eagles) 2021
///
///
///
/// Simple software level that esc launch for drone.
/// Esc motor lauching with selected pwm and test how rotate motors.
///
/// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
/// # Examples
///
/// ```
/// use crate::controller::Controller;
/// let mut controller = Controller::new(); // Initialize class of esc driver
/// controller.calibrate_esc(); // Calibrate esc
/// controller.set_throttle_external_pwm(1050, 1050, 1050, 1050, 1050); // Run esc's motor with selected pusle width
/// ```
///
pub struct Controller {
    pwm: Pca9685<I2cdev>,
}

impl Controller {
    /// Returns Controller object
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// ***** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    /// use crate::controller::Controller;
    /// let mut controller = Controller::new();
    /// ```
    ///
    pub fn new() -> Controller {
        let mut config = config_parser::new();
        let dev = I2cdev::new(config.esc_config_parser().port).unwrap();
        let mut pwm_mod: Pca9685<I2cdev> = Pca9685::new(dev, Address::default()).unwrap();
        pwm_mod.set_prescale(PRESCALER).unwrap(); // you need to set a correct prescaler
        pwm_mod.set_channel_on(Channel::C0, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C1, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C2, 0).unwrap();
        pwm_mod.set_channel_on(Channel::C3, 0).unwrap();
        pwm_mod.enable().unwrap();

        Controller { pwm: pwm_mod }
    }
    /// Launch esc motors by selected pwm duty on each motor
    ///
    /// # Arguments
    ///
    /// ch1 - channel 1
    ///
    /// ch2 - channel 2
    ///
    /// ch3 - channel 3
    ///
    /// ch4 - channel 4
    ///
    /// All values of channels are range from 1000 - 2000 us
    ///
    /// 1000 - slowest speed of running esc
    ///
    /// 2000 - maximun speed of running esc
    ///
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    /// use crate::controller::Controller;
    /// let mut controller = Controller::new(); // create object of Controller
    /// controller.calibrate_esc(); // Calibrate esc. This function is mandatory to use before starting run esc
    /// controller.set_throttle_external_pwm(1050, 1050, 1050, 1050, 1050); // Set specifity pusle of pwm generate esc driver
    /// ```
    ///
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
    /// Testing launching esc motors by selected pwm duty for all motor.
    ///
    /// Value of pmw duty is same for all motors
    ///
    ///  To avoid doesn't work esc, use function of calibarating esc. Call function ```calibrate_esc();``` before launch esc.
    ///
    /// # Arguments
    /// pwm_duty - speed to test how launch esc motors
    ///
    /// 1000 - slowest speed of running esc
    ///
    /// 2000 - maximun speed of running esc
    ///
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    /// use crate::controller::Controller;
    /// let mut controller = Controller::new();
    /// controller.calibrate_esc();
    /// controller.test_esc(1500);
    /// ```
    ///
    pub fn test_esc(&mut self, pwm_duty: u16) {
        self.pwm
            .set_channel_on_off(Channel::C0, 0, pwm_duty)
            .unwrap();
        self.pwm.enable();
    }
    /// Calibration all esc motors
    ///
    /// This fuction give teachering the esc controller what range of output throttle input it should respond to.
    /// A function give range of pulse 1000-2000 to teaching the esc controller.
    /// Before usage esc, you must use this function to works property esc. This function is mandatory!
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    /// use crate::controller::Controller;
    /// let mut controller = Controller::new();
    /// controller.calibrate_esc();
    /// ```
    ///
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
