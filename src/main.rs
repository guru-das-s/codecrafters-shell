#[allow(unused_imports)]
use std::io::{self, Write};

fn is_valid(_s: &String) -> bool {
    false
}

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if !is_valid(&input) {
        println!("{}: command not found", input.trim());
    }
}
