use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Letter;

// Create Game

#[derive(Serialize, Deserialize)]
pub enum GameKind {
    Random,
    Custom,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameParams {
    pub kind: GameKind,
    pub word: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub game: Uuid,
}

// Join Game

#[derive(Serialize, Deserialize)]
pub struct JoinGameParams {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct JoinGameResponse {
    pub player: Uuid,
    pub length: usize,
}

// Create Guess
#[derive(Serialize, Deserialize)]
pub struct PlayerGuessParams {
    pub guess: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerGuess {
    pub letters: Vec<Letter>,
    pub is_winning_guess: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerGuessResponse {
    pub guess: PlayerGuess,
}

// Get Guesses

#[derive(Serialize, Deserialize)]
pub struct PlayerGuessesResponse {
    pub guesses: Vec<PlayerGuess>,
}
