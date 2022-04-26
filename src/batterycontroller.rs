#[allow(non_camel_case_types)]
//
// Misha Zaslavkis (Electrics Eagles) 2022
//
// ---------------- Battery controller --------------------
//
// This software is designed for prevent overdischanging, damaging, fire battery, when battery is used for drone with low voltage
// Battery also prevent lower minimun voltage for LIPO cell, which software will alert when cell is lower voltage than minimun working voltage
// Each cell of LIPO battery is minimun 3.0 volts
// Each cell of LIPO battery is maximum 4.2 volts
// Using LIPO battery, please handle very carefully to prevent serious injury and fire
// typical usage :
// use crate::batterycontroller::*;
// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
// let voltage_lipo_battery = battery.measure_voltage_of_lipo_battery(); // Measure common voltage of lipo battery
// println!("Main battery voltage: {:.2} V", voltage_lipo_battery); // Print voltage in console (OPTIONAL)
// let voltage_first_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Firstcell); // Measure voltage of 1st cell of LIPO battery
// let voltage_second_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Secondcell); // Measure voltage of 2st cell of LIPO battery
// let voltage_third_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Trindcell); // Measure voltage of 3st cell of LIPO battery
// println!("Voltage each cell of LIPO battery, V: C1: {:.2}, C2: {:.2}, C3: {:.2}", voltage_first_cell, voltage_second_cell, voltage_third_cell);
// if battery.some_cell_is_lower_than_min_voltage() == true { println!("Some cell is lower than 3 volts, please land your drone..."); } // Alert if some cells are low voltage
//Enjoy
extern crate embedded_hal;
use embedded_hal::adc::OneShot;
extern crate ads1x1x;
extern crate linux_embedded_hal;
extern crate nb;

use crate::config_parse::*;
use ads1x1x::{channel, Ads1x1x, DataRate16Bit, SlaveAddr};
use linux_embedded_hal::I2cdev;
/// Selection ADC channels enumaration
pub enum ADCCHANNEL {
    /// Selection to measure ADC of channel 0
    Channel0,
    /// Selection to measure ADC of channel 1
    Channel1,
    /// Selection to measure ADC of channel 2
    Channel2,
    /// Selection to measure ADC of channel 3
    Channel3,
}
/// Selection battery cell to measure voltage across selected cell
pub enum BATTERYCELL {
    /// Selection to measure voltage across 1st cell of LIPO battery
    Firstcell,
    /// Selection to measure voltage across 2nd cell of LIPO battery
    Secondcell,
    /// Selection to measure voltage across 3rd cell of LIPO battery
    Trindcell,
}
/// It is a BatteryController object
///
/// BatteryController class (crate)
///
/// Made by: Misha Zaslavskis (Electrics Eagles) 2022
///
/// ---------------- Battery controller --------------------
///
/// This simple software that measuring voltage of LIPO battery and alert of critical dischanging battery
///
/// This software is designed for prevent overdischanging, damaging, fire battery, when battery is used for drone with low voltage
///
/// Battery also prevent lower minimun voltage for LIPO cell, which software will alert when cell is lower voltage than minimun working voltage
///
/// Each cell of LIPO battery is minimun 3.0 volts
///
/// Each cell of LIPO battery is maximum 4.2 volts
///
/// Using LIPO battery, please handle very carefully to prevent serious injury and fire
///
/// NB: I2C address of ADS1115 is set default address, but you cannot change address via core. This default I2C address is 0x48.
///   
/// NB: Please make sure that ADS1115 I2C adress via essesntial pins of ADC chip is set to 0x48.
///
/// NB: Each channel of this ADC chip measure maximum voltage is 2.048V connecting without potential divider, because we set default value FSR (Full-Scale Range) in library of ADS1115
///
///
///
/// # Examples
///
/// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
///
/// ```
/// use crate::batterycontroller::*;
/// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
/// let voltage_lipo_battery = battery.measure_voltage_of_lipo_battery(); // Measure common voltage of lipo battery
/// println!("Main battery voltage: {:.2} V", voltage_lipo_battery); // Print voltage in console (OPTIONAL)
/// let voltage_first_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Firstcell); // Measure voltage of 1st cell of LIPO battery
/// let voltage_second_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Secondcell); // Measure voltage of 2st cell of LIPO battery
////let voltage_third_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Trindcell); // Measure voltage of 3st cell of LIPO battery
/// println!("Voltage each cell of LIPO battery, V: C1: {:.2}, C2: {:.2}, C3: {:.2}", voltage_first_cell, voltage_second_cell, voltage_third_cell);
/// if battery.some_cell_is_lower_than_min_voltage() == true { println!("Some cell is lower than 3 volts, please land your drone..."); } // Alert if some cells are low voltage
/// ```
///
pub struct BatteryController {
    pub adc: ads1x1x::Ads1x1x<
        ads1x1x::interface::I2cInterface<linux_embedded_hal::I2cdev>,
        ads1x1x::ic::Ads1115,
        ads1x1x::ic::Resolution16Bit,
        ads1x1x::mode::OneShot,
    >,
    _max_vol_ch0: f32,
    _max_vol_ch1: f32,
    _max_vol_ch2: f32,
}
impl BatteryController {
    /// Returns BatteryController object
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```BatteryController```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::batterycontroller::*;
    /// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
    /// ```
    ///
    pub fn init() -> BatteryController {
        let mut config = config_parser::new();
        let batt_config = config.battery_controller_parser();
        let dev = I2cdev::new(batt_config.port).unwrap();

        let address = SlaveAddr::default();
        let mut _adc = Ads1x1x::new_ads1115(dev, address);
        _adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
        BatteryController {
            adc: _adc,
            _max_vol_ch0: batt_config.max_vol_ch0,
            _max_vol_ch1: batt_config.max_vol_ch1,
            _max_vol_ch2: batt_config.max_vol_ch2,
        }
    }
    /// Read raw value of selected channel ADC
    ///  
    /// # Arguments
    ///
    /// ```channel``` is selected channel to measure ADC value (ADCCHANNEL)
    ///
    /// # Return
    /// ```i16```
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::batterycontroller::*;
    /// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
    /// let adc_value = battery_cont.read_adc(ADCCHANNEL::Channel3); // Measure ADC value of channel 3
    /// println!("ADC of ch3: {}", adc_value);
    /// ```
    ///
    pub fn read_adc(&mut self, channel: ADCCHANNEL) -> i16 {
        match channel {
            ADCCHANNEL::Channel0 => {
                return nb::block!(self.adc.read(&mut channel::SingleA0)).unwrap()
            }
            ADCCHANNEL::Channel1 => {
                return nb::block!(self.adc.read(&mut channel::SingleA1)).unwrap()
            }
            ADCCHANNEL::Channel2 => {
                return nb::block!(self.adc.read(&mut channel::SingleA2)).unwrap()
            }
            ADCCHANNEL::Channel3 => {
                return nb::block!(self.adc.read(&mut channel::SingleA3)).unwrap()
            }
        }
    }
    /// Measure voltage across selected cell of battery
    ///  
    /// # Arguments
    ///
    /// ```cell``` is selecting position of cell of battery (BATTERYCELL)
    ///
    /// # Return
    /// ```f32``` is value of volatge of selected cell of battery in volts
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::batterycontroller::*;
    /// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
    /// let voltage_first_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Firstcell); // Measure voltage of 1st cell of LIPO battery
    /// let voltage_second_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Secondcell); // Measure voltage of 2st cell of LIPO battery
    /// let voltage_third_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Trindcell); // Measure voltage of 3st cell of LIPO battery
    /// ```
    ///
    pub fn measure_voltage_of_selected_cell(&mut self, cell: BATTERYCELL) -> f32 {
        match cell {
            BATTERYCELL::Firstcell => {
                return self.read_adc(ADCCHANNEL::Channel2) as f32
                    * (self._max_vol_ch2 / (i16::MAX) as f32)
            }

            BATTERYCELL::Secondcell => {
                return self.read_adc(ADCCHANNEL::Channel1) as f32
                    * (self._max_vol_ch1 / (i16::MAX) as f32)
                    - self.read_adc(ADCCHANNEL::Channel2) as f32
                        * (self._max_vol_ch2 / (i16::MAX) as f32)
            }

            BATTERYCELL::Trindcell => {
                return self.read_adc(ADCCHANNEL::Channel0) as f32
                    * (self._max_vol_ch0 / (i16::MAX) as f32)
                    - self.read_adc(ADCCHANNEL::Channel1) as f32
                        * (self._max_vol_ch1 / (i16::MAX) as f32)
            }
        }
    }
    /// Measure voltage LIPO Battery (Across all series cell in LIPO battery)
    ///  
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```f32``` is value of volatge of LIPO battery in volts
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::batterycontroller::*;
    /// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
    /// let voltage_first_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Firstcell); // Measure voltage of 1st cell of LIPO battery
    /// let voltage_second_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Secondcell); // Measure voltage of 2st cell of LIPO battery
    /// let voltage_third_cell = battery.measure_voltage_of_selected_cell(BATTERYCELL::Trindcell); // Measure voltage of 3st cell of LIPO battery
    /// ```
    ///
    pub fn measure_voltage_of_lipo_battery(&mut self) -> f32 {
        return self.read_adc(ADCCHANNEL::Channel0) as f32
            * (self._max_vol_ch0 / (i16::MAX) as f32);
    }
    /// Check that any cells of LIPO battery is low voltage
    ///  
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```bool``` is state of dischanged some cells of LIPO battery
    ///
    /// ```true``` is means that some of cell of LIPO battery is dischanged, which cell is required to charge (Will alert)
    ///
    /// ```false``` is means that all cells of LIPO battery is OKAY, which they follow LIPO range voltage (such not below that 3V each cell of LIPO battery)
    ///
    ///
    /// # Example
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// ```
    /// use crate::batterycontroller::*;
    /// let mut battery_cont = BatteryController::init(); // Initialization battery controller software
    /// if battery.some_cell_is_lower_than_min_voltage() == true { println!("Some cell is lower than 3 volts, please land your drone..."); } // Alerting sample
    /// ```
    ///
    pub fn some_cell_is_lower_than_min_voltage(&mut self) -> bool {
        if self.measure_voltage_of_selected_cell(BATTERYCELL::Firstcell) < 3.0
            || self.measure_voltage_of_selected_cell(BATTERYCELL::Secondcell) < 3.0
            || self.measure_voltage_of_selected_cell(BATTERYCELL::Trindcell) < 3.0
        {
            return true;
        } else {
            return false;
        }
    }
}
