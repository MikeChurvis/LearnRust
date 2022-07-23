// use namespace::module
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Type::associated_fn()
        let mut guess = String::new();

        // Read standard input into the guess string.
        io::stdin()
            // Pass to .read_line() a mutable reference to 'guess'.
            .read_line(&mut guess)
            // If this Result is an Err, crash the program.
            .expect("Failed to read line");

        // Note: .expect() is far less useful than exception handling, taught in ch09.

        // Shadow the previous guess variable, this time with an immutable unsigned
        // 32 bit number. Return to the top of the loop if the input is not a number.
        // Note: .parse() relies on the type annotation to determine its output type.
        // Note: Err(_) matches all errors regardless of content; _ is a catchall.
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match thing { pattern => expression, ... }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
