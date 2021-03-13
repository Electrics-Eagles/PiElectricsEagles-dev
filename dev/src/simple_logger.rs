// Security level : Low. May contain virus
///
///
/// Alex Zaslavkis and Misha Zaslavskis (Electrics Eagles) 2021
///
/// ---------------- Logger --------------------
///
/// Simple software level that write log data. Works with the main code file
///
/// typical usage :
///
/// init_log(); // initiazation log file to property work logger. 
/// write_log(mode: LevelOfLog, msg: String) // write log data
/// logger_verison() // show a version of logger
///
/// Enjoy

use log::*;
use simplelog::*;
use std::fs::{File};
/// This constant is a path of file to save log-data.
///
const FILE_PATH_FOR_LOG: &str = "/var/pielectricseagles/pielectricseagles.log";
/// Enumation of log level
pub enum LevelOfLog {
    ERROR, // error
    INFO, // infomation level
    WARNING, // warning level
    DEBUG, // debug level
    TRACE, // trace level
}
/// initialization log function for write log data
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// No values is returned
    ///
    /// # Examples
    /// *Already added to main file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// use crate::simple_logger::*;
    /// use simple_logger::*;
    /// init_log();
    /// ```
    ///
pub fn init_log() {
    let mut config_log = ConfigBuilder::new(); // create configuration of logger
    config_log.set_time_format(String::from("%F %T")); // select format of date and time
    // Initialize of modes of log 
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::max(), config_log.build(), TerminalMode::Mixed).unwrap(), // Terminal log 
        WriteLogger::new(
            LevelFilter::max(),
            config_log.build(),
            File::create(FILE_PATH_FOR_LOG).unwrap(), // File log
        ),
    ])
    .unwrap();
}
/// Show version of log 
    /// # Arguments
    ///
    /// No arguments required
    ///
    /// # Return
    /// string of version
    /// ```&'static str```
    ///
    /// # Examples
    /// *Already added to main file. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// use crate::simple_logger::*;
    /// use simple_logger::*;
    /// println!("{}", logger_verison());
    /// ```
    ///
pub fn logger_verison() -> &'static str {
    return "LOGGER VER V1.0.2 06/03/2021";
}
/// Write log 
/// Log will show in your terminal when you run this code and it also write file for retriving log data after close this software.
/// 
/// Date and time will write automatically.
/// 
/// Example to show log in terminal:  ```2021-03-03 20:55:58 [ INFO] LOGGER VER V1.0.2 06/03/2021```
    /// # Arguments
    ///
    /// ```mode: LevelOfLog``` Select level of log to show title of this log
    /// 
    /// ```msg: String``` text about log information
    ///
    /// # Return
    /// No values is returned 
    ///
    /// # Examples
    /// *Already added to main and loggics files. Be careful. Editing code can break stability of devices.*
    ///
    /// ```
    /// use crate::simple_logger::*;
    /// use simple_logger::*;
    /// init_log();
    /// write_log(LeveOfLog::INFO, String::from("Gyroscope is OK!"));
    /// write_log(LevelOfLog::ERROR, format!("Initailization ibus error!"));
    /// ```
    ///
pub fn write_log(mode: LevelOfLog, msg: String) {
    match mode {
        LevelOfLog::DEBUG => debug!("{}", msg), // debug title of log line
        LevelOfLog::ERROR => error!("{}", msg), // error title of log line
        LevelOfLog::INFO => info!("{}", msg), //  information title of log line
        LevelOfLog::WARNING => warn!("{}", msg), // warning title of log line
        LevelOfLog::TRACE => trace!("{}", msg) // trace title of log line
    }
}
