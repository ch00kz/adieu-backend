use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    check_guess,
    db::PlayerGuessRecord,
    types::{
        CreateGameParams, CreateGameResponse, GameGuess, GameGuessesResponse, GameKind,
        JoinGameParams, JoinGameResponse, PlayerGuess, PlayerGuessParams, PlayerGuessResponse,
        PlayerGuessesResponse,
    },
};

pub enum JsonResponse<T>
where
    T: Serialize,
{
    JsonOk(StatusCode, T),
    JsonErr(StatusCode, String),
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            JsonResponse::JsonOk(code, json) => (code, Json(json)).into_response(),
            JsonResponse::JsonErr(code, error_message) => {
                (code, Json(json!({"error": error_message}))).into_response()
            }
        }
    }
}

fn json_ok<T: Serialize>(status_code: StatusCode, data: T) -> JsonResponse<T> {
    JsonResponse::JsonOk(status_code, data)
}

fn json_err<T: Serialize>(status_code: StatusCode, error_message: &str) -> JsonResponse<T> {
    JsonResponse::JsonErr(status_code, error_message.to_owned())
}

pub async fn create_game_handler(
    State(pg_pool): State<PgPool>,
    Json(params): Json<CreateGameParams>,
) -> JsonResponse<CreateGameResponse> {
    let word = match params.kind {
        GameKind::Custom => params.word.unwrap().to_uppercase(),
        GameKind::Random => String::from("ADIEU"),
    };

    match super::db::create_game(&pg_pool, word).await {
        Ok(game) => json_ok(StatusCode::CREATED, CreateGameResponse { game }),
        Err(_) => json_err(StatusCode::BAD_REQUEST, "Error creating game"),
    }
}

pub async fn join_game_handler(
    State(pg_pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
    Json(params): Json<JoinGameParams>,
) -> JsonResponse<JoinGameResponse> {
    match super::db::join_game(&pg_pool, game_id, params.username).await {
        Err(_) => json_err(StatusCode::BAD_REQUEST, "Error creating a new player"),
        Ok(player) => match super::db::get_solution(&pg_pool, player).await {
            Err(_) => json_err(StatusCode::BAD_REQUEST, "Error fetching game solution"),
            Ok(solution) => json_ok(
                StatusCode::CREATED,
                JoinGameResponse {
                    player,
                    length: solution.len(),
                },
            ),
        },
    }
}

pub async fn player_guess_handler(
    State(pg_pool): State<PgPool>,
    Path(player_id): Path<Uuid>,
    Json(params): Json<PlayerGuessParams>,
) -> JsonResponse<PlayerGuessResponse> {
    match super::db::get_solution(&pg_pool, player_id).await {
        Err(_) => json_err(StatusCode::BAD_REQUEST, "Error fetching game solution"),
        Ok(solution) => {
            let player_guess = check_guess(&params.guess, &solution);
            match super::db::create_guess(
                &pg_pool,
                player_id,
                params.guess.clone(),
                player_guess.is_winning_guess,
            )
            .await
            {
                Err(_) => json_err(StatusCode::BAD_REQUEST, "Error creating guess"),
                Ok(_) => json_ok(
                    StatusCode::CREATED,
                    PlayerGuessResponse {
                        guess: player_guess,
                    },
                ),
            }
        }
    }
}

pub async fn player_guesses_handler(
    State(pg_pool): State<PgPool>,
    Path(player_id): Path<Uuid>,
) -> JsonResponse<PlayerGuessesResponse> {
    match super::db::get_solution(&pg_pool, player_id).await {
        Err(_) => json_err(StatusCode::BAD_REQUEST, "Error fetching solution"),
        Ok(solution) => match super::db::get_guesses(&pg_pool, player_id).await {
            Err(_) => json_err(StatusCode::BAD_REQUEST, "Error fetching player guesses"),
            Ok(guesses_records) => {
                let player_guesses: Vec<PlayerGuess> = guesses_records
                    .iter()
                    .map(|record| check_guess(&record.guess, &solution))
                    .collect();
                json_ok(
                    StatusCode::OK,
                    PlayerGuessesResponse {
                        guesses: player_guesses,
                    },
                )
            }
        },
    }
}

pub async fn game_guesses_handler(
    State(pg_pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
) -> JsonResponse<GameGuessesResponse> {
    match super::db::get_all_guesses(&pg_pool, game_id).await {
        Err(_) => json_err(StatusCode::BAD_REQUEST, "Error fetching guesses"),
        Ok(records) => {
            let guesses: Vec<GameGuess> = records.into_iter().map(|r| r.into()).collect();
            json_ok(StatusCode::OK, GameGuessesResponse { guesses })
        }
    }
}

impl From<PlayerGuessRecord> for GameGuess {
    fn from(value: PlayerGuessRecord) -> Self {
        GameGuess {
            player: value.player_id,
            username: value.username.clone(),
            guesses: value.guesses.unwrap_or_default(),
            has_won: value.has_won.unwrap_or_default(),
        }
    }
}
