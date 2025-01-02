use std::{collections::HashMap, iter::zip};

use serde::{Deserialize, Serialize};
use types::PlayerGuess;
use typeshare::typeshare;

pub mod db;
pub mod handlers;
pub mod types;

#[typeshare]
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(tag = "status", content = "letter")]
pub enum Letter {
    Correct(char),
    InTheWord(char),
    Wrong(char),
    Unsubmitted(char),
}

fn check_guess(guess: &str, solution: &str) -> PlayerGuess {
    let mut looking_for = HashMap::<char, u32>::new();
    let mut letters = Vec::<Letter>::new();
    // First Pass: Mark correct guesses and wrong guesses. Also make note of what we're looking for.
    for (guess_char, solution_char) in zip(
        guess.to_lowercase().chars(),
        solution.to_lowercase().chars(),
    ) {
        if guess_char == solution_char {
            letters.push(Letter::Correct(guess_char))
        } else {
            *looking_for.entry(solution_char).or_insert(0) += 1;
            letters.push(Letter::Wrong(guess_char))
        }
    }

    // Second Pass: Check if we're looking for any of the wrong guesses, and mark as InTheWord
    letters = letters
        .into_iter()
        .map(|l| match l {
            Letter::Wrong(c) => {
                if let Some(amount_looking_for) = looking_for.get_mut(&c) {
                    if *amount_looking_for > 0 {
                        *amount_looking_for -= 1;
                        return Letter::InTheWord(c);
                    }
                }
                return l;
            }
            _otherwise => return l,
        })
        .collect();

    let is_winning_guess = letters.iter().all(|l| matches!(l, Letter::Correct(_)));
    PlayerGuess {
        letters,
        is_winning_guess,
    }
}
