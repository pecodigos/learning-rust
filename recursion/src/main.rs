use std::io::{self, Write};

fn main() {
    print!("Input your factorial number: ");
    io::stdout().flush().unwrap();

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read line.");

    let number: u64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("The result for {}! is: {}", number, factorial(number));
}


fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    }
    else {
        n * factorial(n - 1)
    }
}