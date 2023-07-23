//! Request and response data structures used by both server and client.

use http::HeaderValue;

#[cfg(feature = "axum")]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::card::Card,
    identifiers::{GameId, PlayerId},
    status::{GameState, GameStatus},
};

/// Token used to uniquely identify each player session.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Token(u64);

impl Token {
    pub fn new(token: u64) -> Self {
        Self(token)
    }
}

/// Data that is passed by requests in `Authorization` header.
///
/// Should be encoded as json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthHeader {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub token: Token,
}

impl AuthHeader {
    pub fn into_header(&self) -> HeaderValue {
        let s = serde_json::to_string(&self).unwrap();
        HeaderValue::from_str(s.as_str()).unwrap()
    }
}

/// Query parameters used to create new game.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateGameData {
    pub password: String,
}

/// Response for [CreateGameData] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum CreateGameResponse {
    Ok {
        game_id: GameId,
        player_id: PlayerId,
        token: Token,
    },
}

#[cfg(feature = "axum")]
impl IntoResponse for CreateGameResponse {
    fn into_response(self) -> Response {
        let code = match self {
            CreateGameResponse::Ok { .. } => StatusCode::OK,
        };
        let body = serde_json::to_string(&self).unwrap();
        (code, body).into_response()
    }
}

/// Query parameters used to join created game.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinGameData {
    pub id: GameId,
    pub password: String,
}

/// Response for [JoinGameData] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum JoinGameResponse {
    /// Joined successfully.
    Ok {
        game_id: GameId,
        player_id: PlayerId,
        token: Token,
    },
    /// Game with provided id doesn't exist.
    NotFound,
    /// Password doesn't match.
    InvalidPassword,
    /// Game is already full.
    TooManyPlayers,
}

#[cfg(feature = "axum")]
impl IntoResponse for JoinGameResponse {
    fn into_response(self) -> Response {
        let code = match &self {
            JoinGameResponse::Ok { .. } => StatusCode::OK,
            JoinGameResponse::NotFound => StatusCode::NOT_FOUND,
            JoinGameResponse::InvalidPassword => StatusCode::BAD_REQUEST,
            JoinGameResponse::TooManyPlayers => StatusCode::BAD_REQUEST,
        };
        let body = serde_json::to_string(&self).unwrap();
        (code, body).into_response()
    }
}

/// Error that occured when attempted to join the game.
#[derive(Debug, Error)]
pub enum JoinGameError {
    #[error("game not found")]
    NotFound,
    #[error("password doesn't match")]
    InvalidPassword,
    #[error("too many players")]
    TooManyPlayers,
}

/// Query parameters used to play card.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayCardData {
    pub player: PlayerId,
    pub card: Card,
}

/// Response to card played.
#[derive(Debug, Serialize, Deserialize)]
pub enum PlayCardResponse {
    Ok,
    InvalidCard,
    NotInHand,
    AuthFailed,
}

#[cfg(feature = "axum")]
impl IntoResponse for PlayCardResponse {
    fn into_response(self) -> Response {
        let code = match &self {
            PlayCardResponse::Ok => StatusCode::OK,
            PlayCardResponse::InvalidCard => StatusCode::BAD_REQUEST,
            PlayCardResponse::NotInHand => StatusCode::BAD_REQUEST,
            PlayCardResponse::AuthFailed => StatusCode::UNAUTHORIZED,
        };
        let body = serde_json::to_string(&self).unwrap();
        (code, body).into_response()
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for GameState {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        (StatusCode::OK, body).into_response()
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for GameStatus {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        (StatusCode::OK, body).into_response()
    }
}
