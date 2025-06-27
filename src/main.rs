use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[allow(dead_code)]
fn hello_world() {
    println!("Hello, world!");
}
fn guess_game() {
    let min = 1;
    let max = 1000;
    println!("Guess the number game!");
    let secret_number = rand::thread_rng().gen_range(min..=max);
    println!("Please enter your guess ({min}-{max}):");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between ({min}-{max}).");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller!, you are around {}% of the way there!", ((secret_number - guess) as f32 / (max - min) as f32 * 100.0).round()),
            Ordering::Greater => println!("Bigger!, you are around {}% of the way there!", ((guess - secret_number) as f32 / (max - min) as f32 * 100.0).round()),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
fn main() {
    guess_game();
}
