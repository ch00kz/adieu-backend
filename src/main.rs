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
    // App State
    let dictionary_path = env::var("DICTIONARY_PATH").expect("DICTIONARY_PATH not set");
    let dictionary = Dictionary::new(&dictionary_path);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pg_pool = PgPool::connect(&db_url)
        .await
        .expect("Could not connect to Database.");
    let app_state = AppState {
        pg_pool,
        dictionary,
    };

    // CORS Layer
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL not set");
    let cors_layer = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

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
        .layer(cors_layer)
        // Add state
        .with_state(Arc::new(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
