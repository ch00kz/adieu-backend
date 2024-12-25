use sqlx::{postgres::PgPool, prelude::FromRow, query, query_as};
use uuid::{self, Uuid};

#[derive(FromRow)]
pub struct GuessRecord {
    pub guess: String,
}

#[derive(FromRow)]
pub struct PlayerGuessRecord {
    pub player_id: Uuid,
    pub username: String,
    pub guesses: Option<i64>,
    pub has_won: Option<bool>,
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
        r#"SELECT guess FROM guesses WHERE player_id = $1 ORDER BY created_at ASC"#,
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

pub async fn get_all_guesses(
    pool: &PgPool,
    game_id: Uuid,
) -> anyhow::Result<Vec<PlayerGuessRecord>, anyhow::Error> {
    Ok(query_as!(
        PlayerGuessRecord,
        r#"SELECT
            p.id as player_id,
            p.username,
            COUNT(g.id) AS guesses,
            BOOL_OR(g.is_winning_guess) AS has_won
        FROM players p
        LEFT JOIN guesses g ON p.id = g.player_id
        WHERE p.game_id = $1
        GROUP BY p.id, p.username
        ORDER BY has_won DESC, guesses ASC
        "#,
        game_id,
    )
    .fetch_all(pool)
    .await?)
}
