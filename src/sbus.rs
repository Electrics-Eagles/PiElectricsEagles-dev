use crate::config_parse::sbus_receiver_conifg;
use rppal::uart::{Parity, Uart};
use std::time::Duration;

pub struct ControllerValues {
    pub ch1: i32,
    pub ch2: i32,
    pub ch3: i32,
    pub ch4: i32,
}
pub fn sbus_uart_init_() -> i32 {
    let values = sbus_receiver_conifg();

    
    return 6;
}
pub fn read_sbus(mut uart: Uart) -> ControllerValues {
    let mut buffer = [0u8; 1];
    loop {
        // Fill the buffer variable with any incoming data.
        if uart.read(&mut buffer).expect("COuld not read uart") > 0 {
            println!("Received byte: {}", buffer[0]);
        }
        let values = ControllerValues {
            ch1: 1000,
            ch2: 1000,
            ch3: 1000,
            ch4: 1000,
        };

        return values;
    }
}
pub fn read_sbus_no() -> ControllerValues {

  
       loop {   // Fill the buffer variable with any incoming data.
        
        let values = ControllerValues {
            ch1: 1000,
            ch2: 1000,
            ch3: 1000,
            ch4: 1000,
        };

        return values;
    }
}
pub fn sbus_verison() -> &'static str {
    return "S_BUS MODULE VERSION  25/09/2020";
}
