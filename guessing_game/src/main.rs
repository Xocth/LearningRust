use std::io; // input/output functionality from the standard library
use std::cmp::Ordering; // p2 // provides comparison functionality
use rand::Rng; // p2 // provides random number generation

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // p2 // generate a random number between 1 and 100

   // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {

    let mut guess = String::new(); // create a mutable String variable to hold user input

    io::stdin()
        .read_line(&mut guess) // read a line from standard input and store it in guess 
        .expect("Failed to read line"); // handle any errors that may occur while reading input

    let guess: u32 = match guess.trim().parse() { // p2 // convert the input string to an unsigned 32-bit integer
        Ok(num) => num, // if parsing is successful, use the number
        Err(_) => continue, // if parsing fails, skip to the next iteration of the loop
        
    };

    println!("You guessed: {guess}"); 

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!"); 
            break; }
        }
    }
}