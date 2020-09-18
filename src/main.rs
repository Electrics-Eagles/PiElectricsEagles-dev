

use crate::config_parser:: get_pids;

#[path = "core/logger/simple_logger.rs"]
mod simple_logger;
#[path = "core/config_parser/config_parse.rs"]
mod config_parser;

#[path = "core/sbus_transmiter/sbus.rs"]
mod sbus;



fn version_display(){
    simple_logger::logger(1, false, simple_logger::logger_verison().parse().unwrap());
    simple_logger::logger(1, false, config_parser::config_parser_version().parse().unwrap());
    simple_logger::logger(1, false,sbus::sbus_verison().parse().unwrap());
    simple_logger::logger(1, false, "SOFTWARE RUN".parse().unwrap());
    simple_logger::logger(1, false, "CONFIG READ&PARSE".parse().unwrap());
}
fn main() {
    version_display(); // call function that display software verison
    let pids_values = get_pids(); // get pid config
    println!("{}",pids_values.get(0).unwrap()[0]);
    simple_logger::logger(1, false, "CONFIG READ&PARSE=OK".parse().unwrap());
    let sbus=config_parser::sbus_receiver_conifg();
    sbus::read_sbus(sbus.baudrate, sbus.parity, sbus.data_bits as u8, sbus.stop_bit as u8, sbus.port).unwrap();



}
