
extern crate hex; 



use rppal::uart::{Parity, Uart}; // crate for uart reading
use std::iter::FromIterator; // std libary for convert into string via vec with char
use std::str;
use std::time::Duration;
use std::vec::Vec;
use mut_static::MutStatic;
use lazy_static::LazyStatic;




pub struct IBus_Uart {
    value: Uart,
}

impl IBus_Uart {
    pub fn new(v: Uart) -> Self {
        IBus_Uart { value: v }
    }
    pub fn getvalue(&self) -> Uart {
        self.value
    }
    pub fn setvalue(&mut self, v: Uart) {
        self.value = v
    }
}

lazy_static! {
    static ref uart_object: MutStatic<IBus_Uart> = MutStatic::new();
}

/* structure for getting a data of all neccessary chanell */
pub struct type_of_data_from_channels {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
    pub ch4: u16,
    pub ch5: u16,
    pub ch6: u16,
}
pub fn init_uart() {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).unwrap();
      // set read mode (maxium value of each reading uart buffer)
    uart.set_read_mode(40, Duration::default()).unwrap();
    uart_object.set(IBus_Uart{value:uart});
}
/* Function for getting data of ibus receiver */
pub fn get_data_of_channel_form_ibus_receiver() -> type_of_data_from_channels {
    // buffer for reading uart before convert into hexidecimal value
    let mut buffer = [0u8; 32];
    // array for getting a data
    let mut data_of_channels: [u16; 6] = [0; 6];
    
    let mut uart_mod=uart_object
    .read()
    .unwrap()
    .getvalue();
    if uart_mod.read(&mut buffer).unwrap() > 0 {
        // encode buffer into hex decimal and convert string
        let input_string_in_hex = hex::encode(buffer);
        // contain chars of hex decimal values by using std::vec
        let input_string_in_char: Vec<char> = input_string_in_hex.chars().collect();
        
        // each channel is size two bytes after welcoming two bytes (0x20 and 0x40)
        // each two bytes need convert opposite site form second byte of each channel into first byte of each channel
        // support maxiumun 14 number of channels
        // starting at 3th and 4th bytes of one reading lenght of ibus, but need set opposite of these bytes that get correctly value of each channel
        // each new channel new number of char
        // iterate six times, because we need to have six channels of receivers
        if input_string_in_char[0] == '2' && input_string_in_char[1] == '0' {
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
    }
    
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
else {
    type_of_data_from_channels {
        ch1: 1000,
        ch2: 1000,
        ch3: 1000,
        ch4: 1000,
        ch5: 1000,
        ch6: 1000,
    }   
}
}