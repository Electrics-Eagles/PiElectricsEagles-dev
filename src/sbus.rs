use std::error::Error;
use std::time::Duration;
use crate::config_parse::sbus_receiver_conifg;
use rppal::uart::{Parity, Uart};

pub fn sbus_uart_init() -> Uart {
    let values=sbus_receiver_conifg();
    
    let mut _parity;
    if values.parity==1 {
        _parity=Parity::Even;
    }
    else {
        _parity=Parity::None;
    }
    let mut uart = Uart::with_path(values.port,values.baudrate, _parity, values.data_bits, values.stop_bit).expect("COULD SET UART");
    uart.set_read_mode(1, Duration::default()).expect("COULD SET UART MODE");
    return uart;
}
pub fn read_sbus( mut uart:Uart) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0u8; 1];
    loop {
        // Fill the buffer variable with any incoming data.
        if uart.read(&mut buffer).expect("COuld not read uart") > 0 {
            println!("Received byte: {}", buffer[0]);
        }
    }

}
pub fn sbus_verison() -> &'static str {
    return "S_BUS MODULE VERSION  25/09/2020";
}
