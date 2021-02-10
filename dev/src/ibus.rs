//
//
// Alex Zaslavkis (Electrics Eagles) 2021
//
// ---------------- CLK Driver --------------------
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
use crate::config_parse::config_parser;
use rppal::uart::Queue::Both;
use rppal::uart::{Parity, Uart}; // crate for uart reading
use std::iter::FromIterator; // std libary for convert into string via vec with char
use std::time::Duration;
use std::vec::Vec;

use crate::simple_logger;

static mut value_before: [u16; 6] = [1000, 1000, 1000, 1000, 1000, 1000];
/// It is a struct for getting data from ibus receiver
/// ibus_receiver class (crate)
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
/// *****Already added to loggics file. Be careful. Editing code can break stability of devices.*****
/// # Examples
///
/// ```
/// let mut reciver_driver = ibus_receiver::new();
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
    /// let mut reciver_driver = ibus_receiver::new();
    /// let reciver = reciver_driver.get_datas_of_channel_form_ibus_receiver();
    /// ```
    ///
    pub fn new() -> ibus_receiver {
        let mut uart_def: Uart = Uart::new(115_200, Parity::None, 8, 1).unwrap();
        simple_logger::logger(1, true, "UART CREATED".parse().unwrap());
        uart_def.set_read_mode(40, Duration::new(1, 7)).unwrap();
        simple_logger::logger(1, true, "UART MODE SET".parse().unwrap());
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
    /// # Examples
    /// **** Already added to loggics file. Be careful. Editing code can break stability of devices. *****
    /// 
    /// ```
    /// let mut reciver_driver = ibus_receiver::new();
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
            //simple_logger::logger(1, true, "DATA READED".parse().unwrap());
            //simple_logger::logger(1, true, std::str::from_utf8(&buffer).unwrap().to_string());
            // encode buffer into hex decimal and convert string
            let input_string_in_hex = hex::encode(buffer);
            // contain chars of hex decimal values by using std::vec
            let input_string_in_char: Vec<char> = input_string_in_hex.chars().collect();
            // each channel is size two bytes after welcoming two bytes (0x20 and 0x40)
            if input_string_in_char[0] == '2'
                && input_string_in_char[1] == '0'
                && input_string_in_char[2] == '4'
                && input_string_in_char[3] == '0'
            {
                // each two bytes need convert opposite site form second byte of each channel into first byte of each channel

                // simple_logger::logger(1, true, "DATA READED AFTER HEX CONVERTION".parse().unwrap());
                //simple_logger::logger(1, true, input_string_in_hex.parse().unwrap());
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
                    simple_logger::logger(1, true, "Hex with replaced value".parse().unwrap());
                    simple_logger::logger(1, true, str_value.parse().unwrap());
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
        }
        self.uart_mod.flush(Both).expect("error");
        // return into struct of data of channels from usual array
        //simple_logger::logger(1, true, String::from_utf16(&data_of_channels).unwrap());
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
