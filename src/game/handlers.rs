use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
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

    // process guess
    let player_guess = check_guess(&params.guess, &solution);

    super::db::create_guess(
        &pg_pool,
        player_id,
        params.guess.clone(),
        player_guess.is_winning_guess,
    )
    .await
    .expect("Expected to save the guess");

    return (
        StatusCode::CREATED,
        Json(PlayerGuessResponse {
            guess: player_guess,
        }),
    );
}

pub async fn player_guesses_handler(
    State(pg_pool): State<PgPool>,
    Path(player_id): Path<Uuid>,
) -> (StatusCode, Json<PlayerGuessesResponse>) {
    let solution = super::db::get_solution(&pg_pool, player_id)
        .await
        .expect("Expected to join a game (create a new player)");

    let guesses_records = super::db::get_guesses(&pg_pool, player_id)
        .await
        .expect("Expected to join a game (create a new player)");

    // process guesses
    let player_guesses: Vec<PlayerGuess> = guesses_records
        .iter()
        .map(|record| check_guess(&record.guess, &solution))
        .collect();

    return (
        StatusCode::OK,
        Json(PlayerGuessesResponse {
            guesses: player_guesses,
        }),
    );
}

pub async fn game_guesses_handler(
    State(pg_pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
) -> (StatusCode, Json<GameGuessesResponse>) {
    let records = super::db::get_all_guesses(&pg_pool, game_id)
        .await
        .expect("Expected to fetch all guesses");

    let guesses = records.iter().filter_map(to_game_guess).collect();

    return (StatusCode::OK, Json(GameGuessesResponse { guesses }));
}

fn to_game_guess(record: &PlayerGuessRecord) -> Option<GameGuess> {
    match (record.guesses, record.has_won) {
        (Some(guesses), Some(has_won)) => Some(GameGuess {
            player: record.player_id,
            username: record.username.clone(),
            guesses,
            has_won,
        }),
        _otherwise => None,
    }
}
