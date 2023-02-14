use std::time::Instant;

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    loop {
    let num = 20;
    let start = Instant::now();
    let result = factorial(num);
    let elapsed = start.elapsed();

    println!("The factorial of {} is {}", num, result);
    println!("Elapsed time: {:?}", elapsed);
}
}
