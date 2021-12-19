#[allow(non_camel_case_types)]
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
use crate::config_parse::config_parser;
/// value for default value of channels when IBUS didn't get signal transmistion
static mut defualt_value: [u16; 14] = [1500; 14];
/// value for returning value from pervious value of channel if checksum was failed
static mut data_of_channels_before: [u16; 14] = [1000; 14]; 

/// It is a struct for getting data from ibus receiver
/// ibus_receiver class (crate)
/// ch1: u16 - value of channel 1 joystick
/// ch2: u16 - value of channel 2 joystick
/// ch3: u16 - value of channel 3 joystick
/// ch4: u16 - value of channel 4 joystick
/// ch5: u16 - value of channel 5 joystick
/// ch6: u16 - value of channel 6 joystick
/// ch7: u16 - value of channel 7 joystick
/// ch8: u16 - value of channel 8 joystick
/// ch9: u16 - value of channel 9 joystick
/// ch10: u16 - value of channel 10 joystick
/// ch11: u16 - value of channel 11 joystick
/// ch12: u16 - value of channel 12 joystick
/// ch13: u16 - value of channel 13 joystick
/// ch14: u16 - value of channel 14 joystick
pub struct type_of_data_from_channels {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
    pub ch4: u16,
    pub ch5: u16,
    pub ch6: u16,
    pub ch7: u16,
    pub ch8: u16,
    pub ch9: u16,
    pub ch10: u16,
    pub ch11: u16,
    pub ch12: u16,
    pub ch13: u16,
    pub ch14: u16,

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
pub struct receiver {
    uart_mod: Uart,
}

impl receiver {
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
    pub fn new() -> receiver {
        let mut config = config_parser::new();
        let config=config.reciver_config();
        let mut uart_def: Uart =
            Uart::with_path(config.uart_port, config.baudrate, Parity::None, 8, 1).unwrap();
        uart_def.set_read_mode(32, Duration::new(0, 7)).unwrap();
        receiver { uart_mod: uart_def }
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
        let mut buffer = [0u8; 32];
        // array for getting a data
        let mut data_of_channels: [u16; 14] = [0; 14];
        // array for resultant data
        let mut resultant_data_of_channels: [u16; 14] = [0; 14];

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
                    // iterate fourteen times, because we need to have six channels of receivers
                    for x in 0..14 {
                        // make reorder each two bytes by using calculating position of char with support this rules of convert bytes in this situation (little endian bytes principle)
                        // we get 16bit value of each channel, so we use 4 characters of 16-bit (2 bytes) hex value in each channel
                        // each new channel, we multiply by 4 of position of bytes, because we use 4 character of each channels
                        // 1 byte is 2 characters of hex value 
                        let ch1_raw_hex = vec![
                            input_string_in_char[6 + (4 * x)],
                            input_string_in_char[7 + (4 * x)],
                            input_string_in_char[4 + (4 * x)],
                            input_string_in_char[5 + (4 * x)],
                        ];
                        // convert into u16 from hex string
                        let value: u16 = u16::from_str_radix(String::from_iter(ch1_raw_hex).as_str(), 16).unwrap();
                        // write a new value of channel into array
                        data_of_channels[x] = value;
                    }
                    
                    // Testing checksum buffer
                    let mut total_bytes: u16 = 0;
                    for index in 0..30 
                    {
                        total_bytes = total_bytes + buffer[index] as u16;
                    } 
                    // Difference between our 0xFFFF and total bytes value to obtain checksum
                    let our_checksum: u16 = 65535 - total_bytes;
                    let real_checksum_raw_hex = String::from_iter(vec![
                        input_string_in_char[62],
                        input_string_in_char[63], 
                        input_string_in_char[60], 
                        input_string_in_char[61]]);

                    let real_checksum_value = u16::from_str_radix(real_checksum_raw_hex.as_str(), 16).unwrap();
                    // checksum check sucessful, data will return from current data of channels-
                    if our_checksum == real_checksum_value 
                    {
                        unsafe {  data_of_channels_before = data_of_channels }
                        resultant_data_of_channels = data_of_channels;
                    }
                    // if checksum failed, data will return from pervious getting data of channels
                    else {
                        unsafe { resultant_data_of_channels = data_of_channels_before; }
                    }

                } else {
                    unsafe { resultant_data_of_channels = data_of_channels_before; }
                }
            } else {
                unsafe { resultant_data_of_channels = data_of_channels_before; }
            }
        }
        //thread::sleep(Duration::from_millis(1));
        self.uart_mod
            .flush(rppal::uart::Queue::Input)
            .expect("error");

        // return into struct of data of channels from usual array
        type_of_data_from_channels {
            ch1: resultant_data_of_channels[0],
            ch2: resultant_data_of_channels[1],
            ch3: resultant_data_of_channels[2],
            ch4: resultant_data_of_channels[3],
            ch5: resultant_data_of_channels[4],
            ch6: resultant_data_of_channels[5],
            ch7: resultant_data_of_channels[6],
            ch8: resultant_data_of_channels[7],
            ch9: resultant_data_of_channels[8],
            ch10: resultant_data_of_channels[9],
            ch11: resultant_data_of_channels[10],
            ch12: resultant_data_of_channels[11],
            ch13: resultant_data_of_channels[12],
            ch14: resultant_data_of_channels[13],
        }
    }
}
