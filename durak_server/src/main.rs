pub mod game;
pub mod state;

use durak_lib::{
    common::{Card, GameId, PlayerId},
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
        .route("/game/:game_id/play", post(play_card))
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

async fn create_game(
    State(games): State<Games>,
    Query(data): Query<CreateGameData>,
) -> impl IntoResponse {
    let game_id = games.create(data.password);
    let player_id = PlayerId::new(0);

    tracing::debug!("created game `{game_id}`");

    CreateGameResponce::Ok {
        token: generate_token(game_id, player_id),
    }
}

async fn join_game(
    State(games): State<Games>,
    Query(data): Query<JoinGameData>,
) -> impl IntoResponse {
    let responce = match games.with_game(data.id, |game| game.join(data.password)) {
        Some(Ok(val)) => Ok(val),
        Some(Err(e)) => Err(e),
        None => Err(JoinGameError::NotFound),
    };

    let responce = match responce {
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
    responce
}

fn generate_token(game_id: GameId, player_id: PlayerId) -> Token {
    Token::new(game_id, player_id, thread_rng().gen())
}

async fn play_card(
    State(games): State<Games>,
    Path(id): Path<GameId>,
    Query(card): Query<Card>,
) -> String {
    games
        .with_game(id, |game| {
            todo!();
        })
        .unwrap_or_else(|| String::from("Game not found"))
}
