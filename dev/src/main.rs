
use loggics::main_loop;


mod clk_driver;
mod config_parse;
mod controller;
mod filter;
mod ibus;
mod l3dgh20;
mod logger;
mod loggics;
mod untils;
mod lis3dh_driver;


fn main() {
    main_loop();
}
