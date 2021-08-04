mod filter;
use filter::low_pass_filter;
use std::fs::File;
use std::{io, env};
use std::io::{BufRead, Write};
use std::path::Path;

#[path = "lib/lib.rs"]
mod lib;
const parse_line: usize = 7;
 
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn parse_log(filename: &str) -> Vec<f64> {
    let size = read_lines(filename).unwrap().count();
    let mut v: Vec<f64> = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        //let mut data:Vec<T>;
        for line in lines {
            if let Ok(line) = line {
                let split = line.split("::");
                let vec = split.collect::<Vec<&str>>();
                let val = vec[parse_line];
                let result = val.to_string().parse::<f64>().unwrap();
                v.push(result);
            }
        }
    }
    return v;
}
fn main() {

    let args: Vec<String> = env::args().collect();


    let delta_ = &args[1].parse::<f64>().unwrap();
    let data = parse_log("pi_drone.log");
    let mut filtered: Vec<f64> = vec![]; //фильтрованое пиво
    let mut pice_of_data: Vec<f64> = vec![]; //фильтрованое пиво
    let mut a = 0.0;
    for ss in 1..100 {
        for i in &data {
            //println!("{}", ss as f64 / 100.0);
            filtered.push(low_pass_filter(*i, *delta_, (ss as f64 / 100.0) as f64));
            pice_of_data.push(a as f64);
            a = a + 1.0;
        }

        lib::plot(&(pice_of_data.clone(), filtered.clone()));
        filtered.clear();
        pice_of_data.clear();
    }
}
