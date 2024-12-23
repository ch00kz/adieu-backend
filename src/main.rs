mod game;
use axum::{
    http::{self, HeaderValue, Method},
    response::Html,
    routing::{get, post},
    Router,
};
use game::handlers::*;
use sqlx::postgres::PgPool;
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pg_pool = PgPool::connect(&db_url)
        .await
        .expect("Could not connect to Database.");

    // build our application with a route
    let app: Router<()> = Router::new()
        // Routes
        .route("/", get(|| async { Html("We did it.") }))
        .route("/game/create", post(create_game_handler))
        .route("/game/:game_id/join", post(join_game_handler))
        .route("/player/:player_id/guess", post(player_guess_handler))
        .route("/player/:player_id/guesses", get(player_guesses_handler))
        // Allow CORS
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        )
        // Add state
        .with_state(pg_pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
