use sqlx::{postgres::PgPool, prelude::FromRow, query, query_as};
use uuid::{self, Uuid};

#[derive(FromRow)]
pub struct GuessRecord {
    pub id: Uuid,
    pub guess: String,
    pub is_winning_guess: bool,
}

pub async fn create_game(pool: &PgPool, word: String) -> anyhow::Result<Uuid> {
    let rec = query!(
        r#"INSERT INTO games ( word ) VALUES ( $1 ) RETURNING id"#,
        word
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn join_game(pool: &PgPool, game_id: Uuid, username: String) -> anyhow::Result<Uuid> {
    let rec = query!(
        r#"INSERT INTO players ( game_id, username ) VALUES ( $1, $2 ) RETURNING id"#,
        game_id,
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn create_guess(
    pool: &PgPool,
    player_id: Uuid,
    guess: String,
    is_winning: bool,
) -> anyhow::Result<Uuid> {
    let rec = query!(
        r#"INSERT INTO guesses ( player_id, guess, is_winning_guess ) VALUES ( $1, $2, $3 ) RETURNING id"#,
        player_id,
        guess,
        is_winning
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn get_guesses(
    pool: &PgPool,
    player_id: Uuid,
) -> anyhow::Result<Vec<GuessRecord>, anyhow::Error> {
    Ok(query_as!(
        GuessRecord,
        r#"SELECT id, guess, is_winning_guess FROM guesses WHERE player_id = $1 ORDER BY created_at ASC"#,
        player_id,
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_solution(pool: &PgPool, player_id: Uuid) -> anyhow::Result<String> {
    let rec = query!(
        r#"SELECT games.word FROM games INNER JOIN players ON players.game_id = games.id WHERE players.id = $1"#,
        player_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.word)
}
