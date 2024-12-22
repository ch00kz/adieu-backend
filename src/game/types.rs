use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
