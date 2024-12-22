use crate::game::types::{CreateGameParams, GameKind};
use sqlx::{postgres::PgPool, query};
use uuid::{self, Uuid};

pub async fn create_game(pool: &PgPool, params: CreateGameParams) -> anyhow::Result<Uuid> {
    let word = match params.kind {
        GameKind::Custom => params.word.unwrap().to_uppercase(),
        GameKind::Random => String::from("ADIEU"),
    };

    let rec = query!(
        r#"INSERT INTO games ( word ) VALUES ( $1 ) RETURNING id"#,
        word
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
