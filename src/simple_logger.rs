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
    #false - is write to file

}

 */



use chrono;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};

pub fn logger_verison() -> &'static str {
    return "LOGGER VER V1.0.1 15/09/2020";
}
pub fn logger(mode : i32, file:bool, msg : String ) {
    let mut _mode_name = "".to_string();
    match mode {
        1=>_mode_name="[INFO]".to_string(),
        2=>_mode_name="[DEBUG]".to_string(),
        3=>_mode_name="[WARN]".to_string(),
        4=>_mode_name="[ERROR]".to_string(),
        5=>_mode_name="[PANIC]".to_string(),

        _ => {}
    }
     if file {
         file_logger(_mode_name,msg);
     }
    else {
        console_logger(_mode_name,msg);
    }
}

fn read_config() -> String {
    let mut conifg = std::fs::File::open("./src/config/logger.config").unwrap();
    let mut config_ = String::new();
    conifg.read_to_string( & mut config_).unwrap();
    print!("The content is: {}", config_);
    let words: Vec<&str> = config_.split("::").collect();
    return words[1].to_string().replace("\n","");
}

pub fn file_logger(mode : String, msg : String) {
    let path = read_config();
    if !std::path::Path::new(&path).exists() {
        File::create(&path).expect("Error create file");

    }
    let mut system_log = OpenOptions::new().append(true).open(path).expect("cannot open file");
    system_log.write_all((mode.to_string() + " " + &*chrono::offset::Utc::now().to_string() + " " + &*msg.to_string()+ &*"\n".to_string()).as_ref()).expect("Error");
}

pub fn  console_logger(mode : String, msg : String ) {
    println!("{}",mode.to_string()+" "+ &*chrono::offset::Utc::now().to_string() +" "+ &*msg.to_string());
}
