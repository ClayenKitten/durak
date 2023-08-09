//! Status of the game.
//!
//! The status is a report of the current state of the game, generated specifically for the player.
//! It only contains data known to the player that requested the report.

pub mod lobby;
pub mod round;

#[cfg(feature = "axum")]
use axum::response::IntoResponse;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    errors::{AuthFailed, GameNotFound},
    identifiers::PlayerId,
};

use self::{lobby::LobbyStatus, round::RoundStatus};

/// Response to the status request.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub enum StatusResponse {
    Lobby(LobbyStatus),
    Round(RoundStatus),
    Finished,
    Error(StatusRequestError),
}

#[cfg(feature = "axum")]
impl IntoResponse for StatusResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
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
pub struct LobbyPlayerData {
    pub id: PlayerId,
    pub name: String,
}
