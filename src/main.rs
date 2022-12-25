// import io library
use std::io;

fn main() {
    // print directions
    println!("Guess the number!");
    println!("Input your guess!");

    // create mutable empty string
    let mut guess = String::new();

    // handle user input
    io::stdin()
        .read_line(&mut guess) //append to guess var
        .expect("failed to read line"); //handle error if present

    // print input
    println!("You guessed: {guess}");
}
