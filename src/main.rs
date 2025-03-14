use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::{Instant, Duration};

fn main() {
    println!("Welcome to the guessing game!");

    let mut high_score: Option<(u32, Duration)> = None; // Store both guess_count and duration

    loop {
        let difficulty = choose_difficulty();
        let (secret_number, range) = generate_secret_number(difficulty);

        println!("I'm thinking of a number between 1 and {} ", range);

        let mut guess_count = 0;
        let start_time = Instant::now();

        // Guessing loop
        loop {
            println!("Please enter your guess: ");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a valid number.");
                    continue;
                }
            };

            guess_count += 1;
            println!("You guessed: {} ", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    let duration = start_time.elapsed();
                    println!(
                        "You win! You took {} guesses and {:?} seconds.",
                        guess_count, duration.as_secs()
                    );

                    if let Some((best_guesses, best_time)) = high_score {
                        if guess_count < best_guesses || (guess_count == best_guesses && duration < best_time) {
                            high_score = Some((guess_count, duration));
                            println!("New high score!");
                        }
                    } else {
                        high_score = Some((guess_count, duration));
                    }

                    break; // Exit guessing loop when the correct number is guessed
                }
            }
        }

        // Ask if the player wants to play again **AFTER WINNING**
        if !play_again() {
            println!("Thanks for playing! Goodbye!");
            break; // Exit game loop if the player chooses "no"
        }
    }
}

// Function to choose difficulty level
fn choose_difficulty() -> u32 {
    loop {
        println!("Please choose a difficulty level:");
        println!("1. Easy (1-50)");
        println!("2. Medium (1-100)");
        println!("3. Hard (1-200)");

        let mut difficulty = String::new();
        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line");

        match difficulty.trim().parse::<u32>() {
            Ok(1) => return 50,
            Ok(2) => return 100,
            Ok(3) => return 200,
            _ => {
                println!("Invalid input, please try again.");
                continue;
            }
        };
    }
}

// Function to generate a random secret number
fn generate_secret_number(range: u32) -> (u32, u32) {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=range);
    (secret_number, range)
}

// Function to ask if the player wants to play again
fn play_again() -> bool {
    loop {
        println!("Do you want to play again? (yes/no)");

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read the line");

        match response.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}
