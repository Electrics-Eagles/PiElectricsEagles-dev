/*
Simple Rust Logger for Drone code. It is really simple ,but helpfull
Write for Alex Zaslavskis

API :
The  Usage :

#[path = "core/logger/simple_logger.rs"] mod simple_logger;

fn main() {

    simple_logger::logger(1, false, "NO ERRORS".parse().unwrap());
    simple_logger::logger(1, true, "NO ERRORS".parse().unwrap());
    #1 is mode
    #false - is write to console

}

 */

use chrono;
use log::*;
use simplelog::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const FILE_PATH_FOR_LOG: &str = "/var/pielectricseagles/pielectricseagles.log";
/* Enumation of modes for looger */
pub enum LevelOfLog {
    ERROR,
    INFO,
    WARNING,
    DEBUG,
    TRACE,
}
pub fn init_log() {
    let mut config_log = ConfigBuilder::new();
    config_log.set_time_format(String::from("%F %T"));
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::max(), config_log.build(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::max(),
            config_log.build(),
            File::create(FILE_PATH_FOR_LOG).unwrap(),
        ),
    ])
    .unwrap();
}

pub fn logger_verison() -> &'static str {
    return "LOGGER VER V1.0.1 15/09/2020";
}

fn read_config() -> String {
    let mut conifg = std::fs::File::open("/etc/pielectricseagles/logger.config").unwrap();
    let mut config_ = String::new();
    conifg.read_to_string(&mut config_).unwrap();
    print!("The content is: {}", config_);
    let words: Vec<&str> = config_.split("::").collect();
    return words[1].to_string().replace("\n", "");
}

pub fn write_log(mode: LevelOfLog, msg: String) {
    match mode {
        LevelOfLog::DEBUG => debug!("{}", msg),
        LevelOfLog::ERROR => error!("{}", msg),
        LevelOfLog::INFO => info!("{}", msg),
        LevelOfLog::WARNING => warn!("{}", msg),
        LevelOfLog::TRACE => trace!("{}", msg),
        _ => println!("No other modes are avialable! ")
    }
}

pub fn console_logger(mode: String, msg: String) {
    println!(
        "{}",
        mode.to_string() + " " + &*chrono::offset::Utc::now().to_string() + " " + &*msg.to_string()
    );
}
