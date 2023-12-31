pub mod auth;
pub mod game;
pub mod state;

use auth::{Authenticate, AuthenticateHost};
use durak_lib::{
    errors::AccessError,
    game::card::Card,
    identifiers::PlayerId,
    network::{CreateGameData, CreateGameResponse, JoinGameData, JoinGameResponse},
    status::{StatusRequestError::GameNotFound, StatusResponse},
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use game::{finished::FinishedState, CardPlayedOutcome, GamePhase};
use state::{Auth, Games};
use std::{mem, net::SocketAddr, str::FromStr};
use tracing::{info, Level};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/create", post(create_game))
        .route("/join", post(join_game))
        .route("/game/start", post(start))
        .route("/game/status", get(status))
        .route("/game/play", post(play_card))
        .route("/game/take", post(take))
        .route("/game/retreat", post(retreat))
        .route("/game/leave", post(leave))
        .fallback(not_found)
        .with_state(AppState::new());

    let addr = match std::env::var("DURAK_SERVER_ADDRESS") {
        Ok(env) => SocketAddr::from_str(&env),
        Err(_) => SocketAddr::from_str(env!("DURAK_SERVER_ADDRESS")),
    }
    .expect("failed to parse listening address");
    info!("listening on {}", addr);

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
    let game_id = games.create(data.name);
    let player_id = PlayerId::new(0);
    let token = auth.generate_token(game_id, player_id);
    auth.store_password(game_id, data.password);
    info!("created game `{game_id}`");

    CreateGameResponse::Ok {
        game_id,
        player_id,
        token,
    }
}

/// Joins already created game.
async fn join_game(
    State(auth): State<Auth>,
    State(games): State<Games>,
    Query(data): Query<JoinGameData>,
) -> impl IntoResponse {
    games
        .with_lobby_game(data.id, |lobby| {
            if !auth.validate_password(data.id, &data.password) {
                info!("attempted to join with wrong password `{}`", data.id);
                return JoinGameResponse::InvalidPassword;
            }

            match lobby.add_player(data.name) {
                Some(player_id) => {
                    info!("player joined game `{}`", data.id);
                    let token = auth.generate_token(data.id, player_id);
                    JoinGameResponse::Ok {
                        game_id: data.id,
                        player_id,
                        token,
                    }
                }
                None => {
                    info!("attempted to join full game `{}`", data.id);
                    JoinGameResponse::TooManyPlayers
                }
            }
        })
        .unwrap_or_else(|err| {
            match err {
                AccessError::AuthFailed(_) => {
                    unreachable!("token auth shouldn't fail for game join request");
                }
                AccessError::GameNotFound(_) => {
                    info!("attempted to join nonexisting game `{}`", data.id);
                }
                AccessError::InvalidPhase(_) => {
                    info!("attempted to join started game `{}`", data.id);
                }
            }
            err.into()
        })
}

/// Starts the game.
///
/// Should be called by game host.
async fn start(
    State(games): State<Games>,
    AuthenticateHost(player): AuthenticateHost,
) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| {
            if game.start() {
                info!("Started game `{}`", player.game_id);
                (StatusCode::OK, "Ok")
            } else {
                (StatusCode::BAD_REQUEST, "Error")
            }
        })
        .unwrap_or((StatusCode::NOT_FOUND, "Game not found"))
}

/// Requests information about [StatusResponse] for the current player.
///
/// Should be called regularly during the game unless it is player's turn.
async fn status(State(games): State<Games>, Authenticate(auth): Authenticate) -> impl IntoResponse {
    games
        .with_game(auth.game_id, |game| match &game.phase {
            GamePhase::Lobby(lobby) => StatusResponse::Lobby(lobby.status()),
            GamePhase::Round(round) => StatusResponse::Round(round.status(auth.player_id)),
            GamePhase::Finished(finished) => StatusResponse::Finished(finished.status()),
        })
        .unwrap_or_else(|not_found| StatusResponse::Error(GameNotFound(not_found)))
}

/// Plays specified card on the table.
async fn play_card(
    Query(card): Query<Card>,
    State(games): State<Games>,
    Authenticate(player): Authenticate,
) -> impl IntoResponse {
    games
        .with_game(player.game_id, |game| {
            if let GamePhase::Round(round) = &mut game.phase {
                match round.play_card(player.player_id, card) {
                    Ok(CardPlayedOutcome::None) => {
                        info!(
                            "card played by player #{} in game `{}`",
                            player.player_id, player.game_id
                        );
                        StatusCode::OK
                    }
                    Ok(CardPlayedOutcome::Win(winner)) => {
                        info!(
                            "card played by player #{} in game `{}` caused win",
                            player.player_id, player.game_id
                        );
                        game.phase = GamePhase::Finished(FinishedState {
                            winner,
                            players: mem::take(&mut round.players),
                        });
                        StatusCode::OK
                    }
                    Err(_) => StatusCode::BAD_REQUEST,
                }
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .unwrap_or(StatusCode::NOT_FOUND);
}

/// Takes all cards from the table into player's hand.
///
/// Should be called by defending player.
async fn take(State(games): State<Games>, Authenticate(player): Authenticate) -> impl IntoResponse {
    games
        .with_started_game(player.game_id, |round| {
            if round.take(player.player_id) {
                info!(
                    "Cards are taken by player #{} in game `{}`",
                    player.player_id, player.game_id
                );
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
        .with_started_game(player.game_id, |round| {
            if round.retreat(player.player_id) {
                info!(
                    "Player #{} retreated in game `{}`",
                    player.player_id, player.game_id
                );
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Leave the game.
// TODO: allow leaving ongoing game.
async fn leave(State(games): State<Games>, Authenticate(player): Authenticate) {
    let _ = games.with_lobby_game(player.game_id, |game| {
        if game.remove_player(player.player_id) {
            info!(
                "player #{} left the game `{}`",
                player.player_id, player.game_id
            );
        }
    });
}
