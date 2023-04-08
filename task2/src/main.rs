use std::io;

fn main() {
    println!("Welcome to the two-player guessing game!");

    println!("Player 1, please enter a number between 1 and 100:");

    let secret_number: u32 = loop {
        let mut secret_number_str = String::new();

        io::stdin()
            .read_line(&mut secret_number_str)
            .expect("Failed to read line");

        match secret_number_str.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => break num,
            _ => println!("Please enter a valid number between 1 and 100!"),
        }
    };

    println!("Player 1 has chosen a number, player 2 can start guessing now!");

    let mut num_guesses = 0;

    loop {
        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                println!("Please enter a valid number between 1 and 100!");
                continue;
            }
        };

        num_guesses += 1;

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                if secret_number % 2 == 0 {
                    println!("Too small! Hint: the secret number is even.");
                } else {
                    println!("Too small! Hint: the secret number is odd.");
                }
            }
            std::cmp::Ordering::Greater => {
                if secret_number % 2 == 0 {
                    println!("Too big! Hint: the secret number is even.");
                } else {
                    println!("Too big! Hint: the secret number is odd.");
                }
            }
            std::cmp::Ordering::Equal => {
                println!(
                    "Congratulations, you guessed the number in {} guesses!",
                    num_guesses
                );
                break;
            }
        }
    }
}