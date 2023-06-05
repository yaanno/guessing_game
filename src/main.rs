use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("*** Guess the number ***");
    println!("Hint: the number is between 1 and 100");

    let mut guesses_left: i32 = 10;

    loop {
        if guesses_left == 0 {
            println!("The secret number was: {secret_number}");
            println!("No more guess, sorry...");
            break;
        }

        println!("You have {guesses_left} guesses left");
        println!("Please input your guess");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                guesses_left = guesses_left - 1;
            }
            Ordering::Greater => {
                println!("Too big");
                guesses_left = guesses_left - 1;
            }
            Ordering::Equal => {
                println!("It's a match!");
                break;
            }
        }
    }
}
