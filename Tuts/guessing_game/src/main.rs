// Bringing input/output (io) from standard (std) library
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // creating a variable that stores input.
    // "mut": mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // take an input & append to a string
                                // "&"mut: argument가 reference라는 뜻. (default: immutable.)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let x = 5;
    let y = 10;

    println!("x ={x} and y + 2 = {}", y+2)
}