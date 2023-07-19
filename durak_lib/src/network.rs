//! Request and responce data structures used by both server and client.

use http::HeaderValue;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Resource;

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
#[cfg_attr(feature = "bevy", derive(Resource))]
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

/// Responce for [CreateGameData] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum CreateGameResponce {
    Ok {
        game_id: GameId,
        player_id: PlayerId,
        token: Token,
    },
}

#[cfg(feature = "axum")]
impl IntoResponse for CreateGameResponce {
    fn into_response(self) -> Response {
        let code = match self {
            CreateGameResponce::Ok { .. } => StatusCode::OK,
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

/// Responce for [JoinGameData] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum JoinGameResponce {
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
impl IntoResponse for JoinGameResponce {
    fn into_response(self) -> Response {
        let code = match &self {
            JoinGameResponce::Ok { .. } => StatusCode::OK,
            JoinGameResponce::NotFound => StatusCode::NOT_FOUND,
            JoinGameResponce::InvalidPassword => StatusCode::BAD_REQUEST,
            JoinGameResponce::TooManyPlayers => StatusCode::BAD_REQUEST,
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

/// Responce to card played.
#[derive(Debug, Serialize, Deserialize)]
pub enum PlayCardResponce {
    Ok,
    InvalidCard,
    NotInHand,
    AuthFailed,
}

#[cfg(feature = "axum")]
impl IntoResponse for PlayCardResponce {
    fn into_response(self) -> Response {
        let code = match &self {
            PlayCardResponce::Ok => StatusCode::OK,
            PlayCardResponce::InvalidCard => StatusCode::BAD_REQUEST,
            PlayCardResponce::NotInHand => StatusCode::BAD_REQUEST,
            PlayCardResponce::AuthFailed => StatusCode::UNAUTHORIZED,
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
