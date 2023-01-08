// import libraries
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // print directions
    println!("Guess the number!");

    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // continue guessing until correct guess
    loop {
        println!("Input your guess!");

        // create mutable empty string
        let mut guess = String::new();

        // handle user input
        io::stdin()
            .read_line(&mut guess) //append to guess var
            .expect("failed to read line"); //handle error if present

        // convert guess from string to number and handle error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print input
        println!("You guessed: {guess}");

        // compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("nailed it");
                break;
            }
        };
    }
}
