use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Guess a number");
    loop {
        println!("Input your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please type in a number.");
                continue;
            }
        };
        println!("You've guessed {}", guess);
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            }
            Ordering::Greater => {
                println!("{}", "Your guess is greater".blue());
                println!("Try again");
                continue;
            }
            Ordering::Less => {
                println!("{}", "Your guess is less".red());
                println!("Try again");
                continue;
            }
        };
    }
}
