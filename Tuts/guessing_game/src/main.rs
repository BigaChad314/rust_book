// Bringing input/output (io) from standard (std) library
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // creating a variable that stores input.
    // "mut": mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}