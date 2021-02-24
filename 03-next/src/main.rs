fn main_1() {
    let mut guess = String::new();
    println!("Hellrustup default stable o world!");
}

// Using a ! means that you’re calling a macro instead of a normal function.
// Compile: rustc main.rs

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // In Rust, variables are immutable by default
    let mut guess = String::new(); // String is UTF-8 encoded

    io::stdin().read_line(&mut guess)
        .expect("Failed to read");
    
    println!("You guessed: {}", guess);
}