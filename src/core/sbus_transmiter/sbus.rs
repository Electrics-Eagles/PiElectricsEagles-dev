use std::error::Error;
use std::time::Duration;

use rppal::uart::{Parity, Uart};

pub fn read_sbus(baudrate:u32,parity:u32,data_bits:u8,stop_bit:u8,port:String) -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(baudrate, Parity::None, data_bits, stop_bit)?;
    println!("{}",port);
    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];
    loop {
        // Fill the buffer variable with any incoming data.
        if uart.read(&mut buffer)? > 0 {
            println!("Received byte: {}", buffer[0]);
        }
    }

}