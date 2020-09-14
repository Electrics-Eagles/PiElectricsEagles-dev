#[path = "core/logger/simple_logger.rs"] mod simple_logger;
#[path = "core/config_parser/config_parse.rs"] mod config_parser;
fn main() {

    simple_logger::logger(1, false, "NO ERRORS".parse().unwrap());
    //simple_logger::logger(1, true, "NO ERRORS".parse().unwrap());
    config_parser::read_config_pid();
}
