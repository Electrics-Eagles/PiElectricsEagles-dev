use crate::config_parser:: get_pids;

#[path = "core/logger/simple_logger.rs"]
mod simple_logger;
#[path = "core/config_parser/config_parse.rs"]
mod config_parser;




fn main() {
    simple_logger::logger(1, false, "SOFTWARE RUN".parse().unwrap());
    simple_logger::logger(1, false, "CONFIG READ&PARSE".parse().unwrap());
    let pids_values = get_pids();
    println!("{}",pids_values.get(0).unwrap()[0]);
    simple_logger::logger(1, false, "CONFIG READ&PARSE=OK".parse().unwrap());
}
