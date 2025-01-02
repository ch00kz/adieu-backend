use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use super::{db::PlayerScoreRecord, Letter};

// Create Game
#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "word")]
pub enum Game {
    Random,
    Custom(String),
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateGameParams {
    pub game: Game,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameResponse {
    pub game_id: Uuid,
}

// Join Game

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinGameParams {
    pub username: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinGameResponse {
    pub player: Uuid,
    pub length: u32,
}

// Create Guess

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlayerGuessParams {
    pub guess: String,
}

// Get Player Guesses

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGuess {
    pub letters: Vec<Letter>,
    pub is_winning_guess: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlayerGuessResponse {
    pub guess: PlayerGuess,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlayerGuessesResponse {
    pub guesses: Vec<PlayerGuess>,
}

// Get Game Guesses

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScore {
    pub player: Uuid,
    pub username: String,
    pub guesses: i32,
    pub has_won: bool,
}

impl From<PlayerScoreRecord> for PlayerScore {
    fn from(value: PlayerScoreRecord) -> Self {
        PlayerScore {
            player: value.player_id,
            username: value.username.clone(),
            guesses: value.guesses.unwrap_or_default() as i32,
            has_won: value.has_won.unwrap_or_default(),
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGameScoresResponse {
    pub player_scores: Vec<PlayerScore>,
}
