#[allow(non_camel_case_types)]
use loggics::main_loop;


#[path = "./drivers/config_parse.rs"]
mod config_parse;
#[path = "./drivers/controller.rs"]
mod controller;
#[path = "./loggics/filter.rs"]
mod filter;
#[path = "drivers/reciver.rs"]
mod reciver;
#[path = "drivers/reciver.rs"]
mod logger;
#[path = "drivers/reciver.rs"]
mod loggics;
#[path = "drivers/reciver.rs"]
mod utils;
#[path = "drivers/reciver.rs"]
mod imu;



fn main() {
    main_loop();
}
