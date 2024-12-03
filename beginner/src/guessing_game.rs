use rand::Rng;
use std::{io, process::Command};

pub fn run() {
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("Guess the number ! \n");
    loop {
        println!("Please input your guess : ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read the input");

        println!("You guessed the number {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        clear_screen();
        if guess == secret_number {
            break;
        }
    }
    println!("correct the secret number was {}", secret_number);
}

fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("not able to clear the screen");
}
