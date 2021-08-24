/*
** io (input/output) library that comes 
** from the std (standard) library
*/
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    println!("Please input your guess.");
    /*
    ** declaring a mutable variable
    ** the :: means new is an associate function
    ** of the string type
    */
    let mut guess = String::new();
    // call the stdin function from the io module
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
