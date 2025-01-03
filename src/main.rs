mod game;
use axum::{
    http::{self, HeaderValue, Method},
    response::Html,
    routing::{get, post},
    Router,
};
use game::{dictionary::Dictionary, handlers::*};
use sqlx::postgres::PgPool;
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

struct AppState {
    pg_pool: PgPool,
    dictionary: Dictionary,
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let frontend_url = match env::var("FRONTEND_URL") {
        Err(_) => "http://localhost:5173",
        Ok(url) => {
            if url.is_empty() {
                "http://localhost:5173"
            } else {
                &url.clone()
            }
        }
    };

    let dictionary = Dictionary::new();
    let pg_pool = PgPool::connect(&db_url)
        .await
        .expect("Could not connect to Database.");
    let app_state = AppState {
        pg_pool,
        dictionary,
    };

    // build our application with a route
    let app: Router<()> = Router::new()
        // Routes
        .route("/", get(|| async { Html("We did it.") }))
        .route("/game", post(create_game_handler))
        .route("/game/{game_id}/join", post(join_game_handler))
        .route("/game/{game_id}/scores", get(get_game_scores_handler))
        .route(
            "/player/{player_id}/guess",
            post(create_player_guess_handler).get(get_player_guesses_handler),
        )
        // Allow CORS
        .layer(
            CorsLayer::new()
                .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        )
        // Add state
        .with_state(Arc::new(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
