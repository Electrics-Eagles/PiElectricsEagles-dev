#[path = "core/logger/simple_logger.rs"] mod simple_logger;

fn main() {

    simple_logger::logger(1, false, "NO ERRORS".parse().unwrap());
    simple_logger::logger(1, true, "NO ERRORS".parse().unwrap())
}
