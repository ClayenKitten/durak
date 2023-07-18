pub mod auth;
pub mod game;
pub mod state;

use auth::Authenticate;
use durak_lib::{
    common::{Card, GameId, PlayerId},
    network::{CreateGameData, CreateGameResponce, JoinGameData, JoinGameError, JoinGameResponce},
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use state::{Auth, Games};
use std::net::SocketAddr;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/create", post(create_game))
        .route("/join", post(join_game))
        .route("/game/start", post(start))
        .route("/game/state", get(state))
        .route("/game/play", post(play_card))
        .route("/game/take", post(take))
        .route("/game/retreat", post(retreat))
        .route("/game/leave", post(leave))
        .fallback(not_found)
        .with_state(AppState::new());

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

/// Creates new game.
async fn create_game(
    State(auth): State<Auth>,
    State(games): State<Games>,
    Query(data): Query<CreateGameData>,
) -> impl IntoResponse {
    let game_id = games.create(data.password);
    let player_id = PlayerId::new(0);

    tracing::debug!("created game `{game_id}`");

    CreateGameResponce::Ok {
        game_id,
        player_id,
        token: auth.generate_token(game_id, player_id),
    }
}

/// Joins already created game.
async fn join_game(
    State(auth): State<Auth>,
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
                game_id: data.id,
                player_id,
                token: auth.generate_token(data.id, player_id),
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

/// Starts the game.
///
/// Should be called by game host.
async fn start(
    State(games): State<Games>,
    Authenticate(player): Authenticate,
) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| {
            if player.player_id != game.host {
                (StatusCode::UNAUTHORIZED, "Only host may start the game")
            } else {
                game.start();
                (StatusCode::OK, "Ok")
            }
        })
        .unwrap_or((StatusCode::NOT_FOUND, "Game not found"))
}

/// Requests information about current [GameState](game::GameState).
///
/// Should be called regularly by client unless server expects action from the client.
async fn state(
    State(games): State<Games>,
    Authenticate(player): Authenticate,
) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| game.state().clone().into_response())
        .unwrap_or((StatusCode::NOT_FOUND, "Game not found").into_response())
}

/// Plays specified card on the table.
async fn play_card(
    Query(card): Query<Card>,
    State(games): State<Games>,
    Authenticate(player): Authenticate,
) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| {
            if game.play_card(player.player_id, card) {
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Takes all cards from the table into player's hand.
///
/// Should be called by defending player.
async fn take(State(games): State<Games>, Authenticate(player): Authenticate) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| {
            if game.take(player.player_id) {
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Discards all cards at the table.
///
/// Should be called by attacking player.
async fn retreat(
    State(games): State<Games>,
    Authenticate(player): Authenticate,
) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| {
            if game.retreat(player.player_id) {
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Leave the game.
async fn leave(State(games): State<Games>, Authenticate(player): Authenticate) {
    games.with_game(player.game_id, |game| {
        if game.remove_player(player.player_id) {
            tracing::debug!("player #{} left the game `{}`", player.player_id, player.game_id);
        }
    });
}
