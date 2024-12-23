use sqlx::{postgres::PgPool, query};
use uuid::{self, Uuid};

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

pub async fn create_guess(pool: &PgPool, player_id: Uuid, guess: String) -> anyhow::Result<Uuid> {
    let rec = query!(
        r#"INSERT INTO guesses ( player_id, guess ) VALUES ( $1, $2 ) RETURNING id"#,
        player_id,
        guess
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
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
