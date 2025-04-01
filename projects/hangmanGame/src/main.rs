mod display;
mod hangman;
use std::io::{self, Write};
use std::process::Command;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
fn wait_for_any_key() {
    print!("\t Press any key to continue...");
    io::stdout().flush().unwrap(); // Make sure the message is printed immediately
    let _ = io::stdin().read_line(&mut String::new()); // Wait for user input
}

fn main() {
    let word = hangman::Hangman::pick_word();
    let mut game = hangman::Hangman::new(word, 6);
    let mut guessed_letters = String::new();
    loop {
        clear_screen();
        display::Display::show_the_cur_stage(game.attempts as usize);
        display::Display::show_the_incomplete_word(&guessed_letters, &game.word);
        let guess = hangman::Hangman::get_guess();
        if guessed_letters.contains(&guess) {
            println!("\t You already guessed that letter!");
            wait_for_any_key();
            continue;
        }
        guessed_letters.push_str(&guess);
        if game.guess(guess.chars().next().unwrap()) {
            println!("\t Good guess!");
            wait_for_any_key();
        } else {
            println!("\t Wrong guess!");
            wait_for_any_key();
        }
        if game.is_game_over() {
            clear_screen();
            display::Display::show_the_cur_stage(game.attempts as usize);
            display::Display::show_the_incomplete_word(&guessed_letters, &game.word);
            println!("\t you lost that game \n the word: {}", game.word);
            break;
        } else if game.is_winner(guessed_letters.as_str()) {
            clear_screen();
            display::Display::show_the_cur_stage(game.attempts as usize);
            display::Display::show_the_incomplete_word(&guessed_letters, &game.word);
            println!(" \t you won the game");
            break;
        }
    }
}
