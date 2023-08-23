//! Status of the game.
//!
//! The status is a report of the current state of the game, generated specifically for the player.
//! It only contains data known to the player that requested the report.

pub mod finished;
pub mod lobby;
pub mod round;

#[cfg(feature = "axum")]
use axum::{response::IntoResponse, Json};
#[cfg(feature = "axum")]
use http::StatusCode;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    errors::{AuthFailed, GameNotFound},
    identifiers::PlayerId,
};

use self::{finished::FinishedStatus, lobby::LobbyStatus, round::RoundStatus};

/// Response to the status request.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub enum StatusResponse {
    Lobby(LobbyStatus),
    Round(RoundStatus),
    Finished(FinishedStatus),
    Error(StatusRequestError),
}

#[cfg(feature = "axum")]
impl IntoResponse for StatusResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            StatusResponse::Lobby(_) => StatusCode::OK,
            StatusResponse::Round(_) => StatusCode::OK,
            StatusResponse::Finished(_) => StatusCode::OK,
            StatusResponse::Error(ref error) => match error {
                StatusRequestError::GameNotFound(_) => StatusCode::NOT_FOUND,
                StatusRequestError::AuthFailed(_) => StatusCode::UNAUTHORIZED,
            },
        };
        (status_code, Json(self)).into_response()
    }
}

/// Error that occurred when trying to get status of the game.
#[derive(Debug, Serialize, Deserialize, Error)]
pub enum StatusRequestError {
    #[error("{0}")]
    GameNotFound(#[from] GameNotFound),
    #[error("auth failed: {0}")]
    AuthFailed(#[from] AuthFailed),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: PlayerId,
    pub name: String,
}
