use std::{collections::HashMap, iter::zip};

use serde::{Deserialize, Serialize};

pub mod db;
pub mod handlers;
pub mod types;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status {
    Correct,
    InTheWord,
    Wrong,
}

#[derive(Serialize, Deserialize)]
pub struct Letter {
    pub status: Status,
    pub letter: char,
}

fn check_guess(guess: &str, solution: &str) -> Vec<Letter> {
    let mut looking_for = HashMap::<char, u32>::new();
    let mut letters = Vec::<Letter>::new();
    // First Pass: Mark correct guesses and wrong guesses. Also make note of what we're looking for.
    for (guess_char, solution_char) in zip(
        guess.to_lowercase().chars(),
        solution.to_lowercase().chars(),
    ) {
        if guess_char == solution_char {
            letters.push(Letter {
                status: Status::Correct,
                letter: guess_char,
            })
        } else {
            *looking_for.entry(solution_char).or_insert(0) += 1;
            letters.push(Letter {
                status: Status::Wrong,
                letter: guess_char,
            })
        }
    }

    // Second Pass: Check if we're looking for any of the wrong guesses, and mark as InTheWord
    for letter in letters.iter_mut().filter(|l| l.status == Status::Wrong) {
        if let Some(amount_looking_for) = looking_for.get_mut(&letter.letter) {
            if *amount_looking_for > 0 {
                letter.status = Status::InTheWord;
                *amount_looking_for -= 1;
            }
        }
    }

    letters
}
