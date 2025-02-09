use rand::prelude::*;
use std::io::{self, Write};
fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<u8> = (1..100).collect();
    nums.shuffle(&mut rng);
    let random_number = nums.choose(&mut rng).unwrap();

    println!("Welcome to the guessing game!");
    println!("You should guess a number between 1 and 100.");
    println!("First, select a difficulty level:");
    println!("1. Easy (10 tries)");
    println!("2. Medium (5 tries)");
    println!("3. Hard (3 tries)");
    let mut difficult_level = String::new();
    io::stdin().read_line(&mut difficult_level).unwrap();

    let tries = match difficult_level.trim() {
        "1" => 10,
        "2" => 5,
        "3" => 3,
        _ => panic!("Invalid difficulty level"),
    };
    println!("You have {} tries", tries);
    println!("Let's start the game!");

    println!("{}", random_number);
    for n in 0..tries {
        let mut user_guess = String::new();

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_guess).unwrap();
        let numeric_guess: u8 = user_guess
            .trim()
            .parse()
            .expect("Por favor ingresa un valor numerico válido");

        if *random_number > numeric_guess {
            println!("Incorrect! the value is greater than {}", numeric_guess);
        }
        if *random_number < numeric_guess {
            println!("Incorrect! the value is less than {}", numeric_guess);
        }
        if *random_number == numeric_guess {
            println!(
                "Correct!, you guessed the correct number at {} attempts",
                n + 1
            );
            break;
        }
    }
}
