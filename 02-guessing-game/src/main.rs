// Using a ! means that you’re calling a macro instead of a normal function.
// Compile: rustc main.rs

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut attempts = 1;

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default
        let mut guess = String::new(); // String is UTF-8 encoded

        io::stdin()
            .read_line(&mut guess) // Returns io::Result
            .expect("Failed to read line");

        // Can "shadow" the previous value of guess with a new one.
        // Used in situations in which you want to convert a value from one type to another type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // Switching from an expect call to a match expression is how you generally
            // move from crashing on an error to handling the error.
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed the number after {} attempts!", attempts);
                break;
            }
        }

        println!("You guessed: {}", guess);
        attempts += 1;
    }

    println!("The secret number is: {}", secret_number);
}
