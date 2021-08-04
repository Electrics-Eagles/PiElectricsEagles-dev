//
//
// Misha Zaslavkis (Electrics Eagles) 2021
//
// ---------------- I-BUS Driver --------------------
//
// Simple software level that will receiving data from RC-controller via IBUS interface
//
// typical usage :
// use crate::ibus::*;
// let mut reciver_driver = ibus_receiver::new();
// let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();
// let channel_6_value = reciver.ch6;
//
// Enjoy

extern crate hex; // crate for convert from u8 to hex
use rppal::uart::{Parity, Uart}; // crate for uart reading
use std::iter::FromIterator; // std libary for convert into string via vec with char 
use std::time::Duration;
use std::vec::Vec;

/// value for default value of channels when IBUS didn't get signal transmistion
static mut value_before: [u16; 6] = [1000, 1000, 1000, 1000, 1000, 1000];
/// It is a struct for getting data from ibus receiver
/// ibus_receiver class (crate)
/// ch1: u16 - value of channel 1 joystick
/// ch2: u16 - value of channel 2 joystick
/// ch3: u16 - value of channel 3 joystick
/// ch4: u16 - value of channel 4 joystick
/// ch5: u16 - value of channel 5 joystick
/// ch6: u16 - value of channel 6 joystick
///
pub struct type_of_data_from_channels {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
    pub ch4: u16,
    pub ch5: u16,
    pub ch6: u16,
}
/// It is a ibus_receiver object
/// ibus_receiver class (crate)

///
/// Made by : Misha Zaslavkis (Electrics Eagles) 2021
///
///
/// Get data about direction of each joysticks, when you control RC-transsmit
///
/// *****Already added to loggics file. Be careful. Editing code can break stability of devices.*****
/// # Examples
///
/// ```
/// use crate::ibus::*;
/// let mut reciver_driver = ibus_receiver::new(); // Initialize class of ibus
/// let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver(); // Gets data of channels
/// ```
///
pub struct ibus_receiver {
    uart_mod: Uart,
}

impl ibus_receiver {
    /// Returns ibus_receiver object
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    /// use crate::ibus::*;
    /// let mut reciver_driver = ibus_receiver::new();
    /// ```
    ///
    pub fn new() -> ibus_receiver {
        let mut uart_def: Uart =
            Uart::with_path("/dev/ttyUSB0", 115_200, Parity::None, 8, 1).unwrap();
        uart_def.set_read_mode(32, Duration::new(0, 7)).unwrap();
        ibus_receiver { uart_mod: uart_def }
    }

    /// Function for getting data of ibus receiver
    ///
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// ```type_of_data_from_channels```
    ///
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    ///
    /// ```
    /// use crate::ibus::*;
    /// let mut reciver_driver = ibus_receiver::new();
    /// let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();
    /// ```
    ///
    pub fn get_datas_of_channel_form_ibus_receiver(&mut self) -> type_of_data_from_channels {
        // buffer for reading uart before convert into hexidecimal value
        let mut _value_before: [u16; 6] = [0; 6];
        let mut buffer = [0u8; 32];
        // array for getting a data
        let mut data_of_channels: [u16; 6] = [0; 6];
        // reading a buffer from ibus intefance
        if self.uart_mod.read(&mut buffer).unwrap() > 0 {
            // encode buffer into hex decimal and convert string
            let input_string_in_hex = hex::encode(buffer);
            // contain chars of hex decimal values by using std::vec
            let input_string_in_char: Vec<char> = input_string_in_hex.chars().collect();
            if input_string_in_char.len() > 0 {
                // each channel is size two bytes after welcoming two bytes (0x20 and 0x40)
                if input_string_in_char[0] == '2'
                    && input_string_in_char[1] == '0'
                    && input_string_in_char[2] == '4'
                    && input_string_in_char[3] == '0'
                {
                    if self.uart_mod.input_len().unwrap() > 0 {
                        self.uart_mod
                            .flush(rppal::uart::Queue::Input)
                            .expect("error");
                    }
                    // each two bytes need convert opposite site form second byte of each channel into first byte of each channel
                    // support maxiumun 14 number of channels
                    // starting at 3th and 4th bytes of one reading lenght of ibus, but need set opposite of these bytes that get correctly value of each channel
                    // each new channel new number of char
                    // iterate six times, because we need to have six channels of receivers
                    for x in 0..6 {
                        // make reorder each two bytes by using calculating position of char with support this rules of convert bytes in this situation
                        // we get 16bit value of each channel, so we use 4 bytes of characters of each channel
                        // each new channel, we multiply by 4 of position of bytes, because we use 4 character of each channels
                        // 1 byte is 2 characters for get value of channel
                        let ch1_raw_hex = vec![
                            input_string_in_char[6 + (4 * x)],
                            input_string_in_char[7 + (4 * x)],
                            input_string_in_char[4 + (4 * x)],
                            input_string_in_char[5 + (4 * x)],
                        ];
                        // convert string with real hex value
                        let str_value = String::from_iter(ch1_raw_hex);
                        // convert into u16 from hex string
                        let value: u16 = u16::from_str_radix(str_value.as_str(), 16).unwrap();
                        // write a new value of channel into array
                        data_of_channels[x] = value;
                    }
                    unsafe {
                        value_before = data_of_channels;
                    }
                } else {
                    unsafe {
                        data_of_channels = value_before;
                    }
                }
            } else {
                unsafe {
                    data_of_channels = value_before;
                }
            }
        }
        //thread::sleep(Duration::from_millis(1));
        self.uart_mod
            .flush(rppal::uart::Queue::Input)
            .expect("error");
        // return into struct of data of channels from usual array
        type_of_data_from_channels {
            ch1: data_of_channels[0],
            ch2: data_of_channels[1],
            ch3: data_of_channels[2],
            ch4: data_of_channels[3],
            ch5: data_of_channels[4],
            ch6: data_of_channels[5],
        }
    }
}
