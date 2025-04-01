#![allow(dead_code)]

pub struct Display;
use serde_json::from_str;
use std::fs;

impl Display {
    pub fn show_the_cur_stage(index: usize) {
        let file_content = fs::read_to_string("hangman_stages.json").expect("Unable to read file");
        let stages = from_str::<Vec<String>>(&file_content).expect("Unable to parse JSON");
        if index < stages.len() {
            println!("{}", stages[index]);
        } else {
            println!("No more stages available.");
        }
    }

    pub fn show_the_incomplete_word(user_gussed_letters: &str, word: &str) {
        print!("[ ");
        for c in word.chars() {
            if !user_gussed_letters.contains(c) {
                print!(" _ ");
            } else {
                print!(" {} ", c);
            }
        }
        println!("]");
    }
}
