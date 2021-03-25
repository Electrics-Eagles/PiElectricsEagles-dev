//
//
// Alex Zaslavkis (Electrics Eagles) 2021
//
// ---------------- CLK Driver --------------------
//
// Simple software level that will produce the signal on CLK pin. Works with the main code file
//
// typical usage :
//
// 		set_pin_clk_high(); // set pin high
//
// 		set_pin_clk_low();; set pin low
//
// 		new(); create new exemplar of driver.
//
// 		clk_driver_ver(); // displays driver version
//
//
//
// According our rule : 27012021 the when loop starts the pin should be low after high.
//
// Enjoy

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

/// This constant is a port where is a CLK pin connected.
///
const CLK_GPIO: u8 = 27; // CLK Pin constant

/// It is a ClkDriver object
/// ClkDriver class (crate)

///
/// Made by : Alex Zaslavkis (Electrics Eagles) 2021
///
///
///
/// Simple software level that will produce the signal on CLK pin. Works with the main code file
///
///
/// * According our rule : 27012021 the when loop starts the pin should be low . After loop is done pin should be  high. *
///
///
///
/// *Already added to loggics file. Be careful. Editing code can break stability of devices.*
/// # Magic constants
/// ```
///const CLK_GPIO: u8 = 23;
/// ```
/// This constant is a port where is a CLK pin connected.
///
/// # Examples
///
///
/// ```
///
/// use crate::clk_driver::ClkDriver;
/// let mut clk_driver=ClkDriver::new();
/// clk_driver.set_pin_clk_high(); // set 1 to digital clk port
///
/// ```
///
pub struct ClkDriver {
    pin: OutputPin,
}

impl ClkDriver {
    /// Returns ClkDriver object
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    ///
    /// use crate::clk_driver::ClkDriver;
    /// let mut clk_driver=ClkDriver::new();
    /// ```
    ///

    pub fn new() -> ClkDriver {
        print!("{}", "CLK Driver loaded 27/01/2021");
        let mut pin = Gpio::new().unwrap().get(CLK_GPIO).unwrap().into_output();
        let clk_struct = ClkDriver { pin };
        return clk_struct;
    }

    /// Set`s pin high
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    ///
    /// use crate::clk_driver::ClkDriver;
    /// let mut clk_driver=ClkDriver::new();
    /// clk_driver.set_pin_clk_high();
    /// ```
    ///
    pub fn set_pin_clk_high(&mut self) {
        self.pin.set_high();
    }

    /// Set`s pin low
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    ///
    /// use crate::clk_driver::ClkDriver;
    /// let mut clk_driver=ClkDriver::new();
    /// clk_driver.set_pin_clk_low();
    /// ```
    ///
    pub fn set_pin_clk_low(&mut self) {
        self.pin.set_low();
    }
    /// Prints out version of code
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    pub fn clk_driver_ver() -> &'static str {
        return "MPU6050 DRIVER  V0.0.1 verison is 127/01/2021 ID is: aTfkned";
    }
}
