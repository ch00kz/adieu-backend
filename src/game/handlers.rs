use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    check_guess,
    types::{
        CreateGameParams, CreateGameResponse, GameKind, JoinGameParams, JoinGameResponse,
        PlayerGuessParams, PlayerGuessResponse,
    },
};

pub async fn create_game_handler(
    State(pg_pool): State<PgPool>,

    Json(params): Json<CreateGameParams>,
) -> (StatusCode, Json<CreateGameResponse>) {
    let word = match params.kind {
        GameKind::Custom => params.word.unwrap().to_uppercase(),
        GameKind::Random => String::from("ADIEU"),
    };

    let game = super::db::create_game(&pg_pool, word)
        .await
        .expect("Expected to create a Game");
    return (StatusCode::CREATED, Json(CreateGameResponse { game }));
}

pub async fn join_game_handler(
    State(pg_pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
    Json(params): Json<JoinGameParams>,
) -> (StatusCode, Json<JoinGameResponse>) {
    let player = super::db::join_game(&pg_pool, game_id, params.username)
        .await
        .expect("Expected to join a game (create a new player)");

    let length = super::db::get_solution(&pg_pool, player)
        .await
        .expect("Expected to join a game (create a new player)")
        .len();
    return (
        StatusCode::CREATED,
        Json(JoinGameResponse { player, length }),
    );
}

pub async fn player_guess_handler(
    State(pg_pool): State<PgPool>,
    Path(player_id): Path<Uuid>,
    Json(params): Json<PlayerGuessParams>,
) -> (StatusCode, Json<PlayerGuessResponse>) {
    let solution = super::db::get_solution(&pg_pool, player_id)
        .await
        .expect("Expected to join a game (create a new player)");

    super::db::create_guess(&pg_pool, player_id, params.guess.clone())
        .await
        .expect("Expected to save the guess");

    // process guess
    let letters = check_guess(&params.guess, &solution);
    return (StatusCode::CREATED, Json(PlayerGuessResponse { letters }));
}
