use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("It's a guessing game!");

    print!("Input the number between 1 and 5: ");
    let mut guess = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess).expect("Failed to read the line.");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!{"Enter the valid number."};
            return;
        }
    };
    let number = rand::thread_rng().gen_range(1..5);

    match guess.cmp(&number) {
        Ordering::Less => println!("Your number is to low! It was {}", number),
        Ordering::Greater => println!("Your number is too big! It was {}", number),
        Ordering::Equal => println!("You guessed! It was {}", number),
    }
}
