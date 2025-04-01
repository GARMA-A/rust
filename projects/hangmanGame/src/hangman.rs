#![allow(dead_code)]

use rand::Rng;
use serde_json::from_str;
use std::fs;

pub struct Hangman {
    pub word: String,
    pub attempts: u32,
    pub max_attempts: u32,
}

impl Hangman {
    pub fn new(word: String, max_attempts: u32) -> Self {
        Hangman {
            word,
            attempts: 0,
            max_attempts,
        }
    }

    pub fn guess(&mut self, letter: char) -> bool {
        if self.word.contains(letter) {
            true
        } else {
            self.attempts += 1;
            false
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }

    pub fn is_winner(&self, guessed_letters: &str) -> bool {
        self.word.chars().all(|c| guessed_letters.contains(c))
    }
    pub fn pick_word() -> String {
        let mut rng = rand::thread_rng();
        let file_content = fs::read_to_string("languages.json").expect("Unable to read file");
        let words = from_str::<Vec<String>>(&file_content).expect("Unable to parse JSON");
        let num = rng.gen_range(0..words.len());
        words[num].clone()
    }

    pub fn get_guess() -> String {
        let mut guess = String::new();
        println!("Please enter a letter: ");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess.trim().to_string()
    }
}
