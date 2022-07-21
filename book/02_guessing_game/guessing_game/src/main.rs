// use namespace::module
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // Type::associated_fn()

    // module::function()
    io::stdin()
        .read_line(&mut guess) // Pass to .read_line() a mutable reference to 'guess'.
        .expect("Failed to read line"); // If this Result is an Err, crash the program and print this message.

    println!("You guessed: {guess}");
}
