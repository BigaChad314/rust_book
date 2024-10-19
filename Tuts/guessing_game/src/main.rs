// Bringing input/output (io) from standard (std) library
use std::io;
use rand::Rng;
use std::cmp::Ordering; // for enum. 값 비교: less, greater, equal.


fn main() {
    println!("Guess the number!");

    // Part 2. Generating random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // creating a variable that stores input.
    // "mut": mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // take an input & append to a string
                                // "&"mut: argument가 reference라는 뜻. (default: immutable.)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // input을 u32 int. 로 바꾼다.

    println!("You guessed: {}", guess);

    // Part 3. Matching guess
    match guess.cmp(&secret_number) { // 'guess' 입력 값과 'secret_number'를 비교. 
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}