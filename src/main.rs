use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let mut rng = thread_rng();
    let n: u8 = rng.gen_range(0..=10); // Inclusive Range
    println!("Hello, world!{}", n);
    request_input();
}

fn request_input() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_size) => resolve_input(&input.trim().to_owned()),
        Err(error) => println!("error: {error}"),
    }
}

fn resolve_input(input: &str) {
    let selection = input.parse::<u8>().expect("Error. Provide a valid number!");
    match selection {
        1 => println!("1"),
        _ => {
            println!("{input} is not an option. Try again.");
            request_input();
        }
    }
}
