use colored::Colorize;
use std::io::{self, Write};

fn main() {
    let word = "trait";
    let input = io::stdin();

    for _ in 1..6 {
        let mut user_input = String::new();

        input
            .read_line(&mut user_input)
            .expect("Failed to provide input");

        for (word_char, user_char) in word.chars().zip(user_input.trim().chars().take(5)) {
            if word_char == user_char {
                print!("{}!", format!(" {user_char} ").on_green())
            } else if word.contains(user_char) {
                print!("{}!", format!(" {user_char} ").on_yellow())
            } else {
                print!("{}!", format!(" {user_char} ").on_black())
            }

            io::stdout().flush().unwrap();
        }
        println!();
        if word == user_input.trim() {
            println!("You got it! The word is {word}");
            break;
        }
    }
}
