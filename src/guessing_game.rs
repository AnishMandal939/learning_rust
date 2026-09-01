use std::cmp::Ordering;
use rand::RngExt;
use colored::Colorize;


pub fn guessing_logic () {
    println!("Guess the number!");
    colored::control::set_override(true);

    let lower_bound: u32 = 1;
    let upper_bound: u32 = 100;

    let secret_number = rand::rng().random_range(lower_bound..=upper_bound);

    println!("The secret number is: {}", secret_number.to_string().yellow().bold());

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

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