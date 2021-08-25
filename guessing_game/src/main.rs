/*
** io (input/output) library that comes 
** from the std (standard) library
** from std import io
*/
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);//or 1..=100
    //println!("The secret number is {}", secret_number);
    let mut i = 1;
    loop {
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
        /* 
        ** Casting guess (string to unsigned 32-bit number)
        ** Using match to see if was typed a integer,
        ** and if not, the program continues at the beginning
        ** of the loop
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("You guessed: {}\nTry number {}", guess, i);
        // Comparing guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        i += 1;
    }
}
