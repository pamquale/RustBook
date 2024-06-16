use std::io::{self, Write};
use rand::Rng;


fn main() {
    println!("Hello, world! It's a guessing game!");

    let mut guess = String::new();
    print!("Enter a number between 1 and 5: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess).expect("Failed to read the line.");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input the valid number");
            return;
        }
    };
    let number = rand::thread_rng().gen_range(1..5);
    if guess == number {
        println!("You guessed, it's {}!", number);
    } else {
        println!("You are wrong. It's {}", number);
    }
}
