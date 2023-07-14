pub mod game;
pub mod state;

use durak_lib::{
    common::{Card, PlayerId},
    network::{
        CreateGameData, CreateGameResponce, JoinGameData, JoinGameError, JoinGameResponce, Token,
    },
};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use rand::{thread_rng, Rng};
use state::Games;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/create", post(create_game))
        .route("/join", post(join_game))
        .fallback(not_found)
        .with_state(Games::default());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn create_game(State(games): State<Games>, Query(data): Query<CreateGameData>) -> String {
    let game_id = games.create(data.password);
    tracing::debug!("created game `{game_id}`");
    let player_id = PlayerId::new(0);
    serde_json::to_string(&CreateGameResponce::Ok {
        token: generate_token(game_id, player_id),
    })
    .unwrap()
}

async fn join_game(State(games): State<Games>, Query(data): Query<JoinGameData>) -> String {
    let responce = match games.join(data.id, data.password) {
        Ok(player_id) => {
            tracing::debug!("joined game `{}`", data.id);
            JoinGameResponce::Ok {
                token: generate_token(data.id, player_id),
            }
        }
        Err(JoinGameError::NotFound) => {
            tracing::debug!("attempted to join nonexisting game `{}`", data.id);
            JoinGameResponce::NotFound
        }
        Err(JoinGameError::InvalidPassword) => {
            tracing::debug!("attempted to join with wrong password`{}`", data.id);
            JoinGameResponce::InvalidPassword
        }
        Err(JoinGameError::TooManyPlayers) => {
            tracing::debug!("attempted to join full game `{}`", data.id);
            JoinGameResponce::TooManyPlayers
        }
    };
    serde_json::to_string(&responce).unwrap()
}

fn generate_token(game_id: u64, player_id: PlayerId) -> Token {
    Token::new(game_id, player_id, thread_rng().gen())
}
