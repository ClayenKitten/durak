use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::errors::AccessError;

use super::LobbyPlayerData;

/// Status of the preparing game.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub struct LobbyStatus {
    /// List of joined players.
    pub players: Vec<LobbyPlayerData>,
}

impl LobbyStatus {
    /// Returns `true` if the game may be started by the host.
    pub fn can_start(&self) -> bool {
        // TODO: allow more than two players.
        self.players.len() == 2
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LobbyStatusResponse {
    Ok(LobbyStatus),
    /// Game is not preparing.
    AccessError(AccessError),
}

impl From<AccessError> for LobbyStatusResponse {
    fn from(value: AccessError) -> Self {
        LobbyStatusResponse::AccessError(value)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for LobbyStatusResponse {
    fn into_response(self) -> Response {
        let code = match self {
            LobbyStatusResponse::Ok(_) => StatusCode::OK,
            LobbyStatusResponse::AccessError(_) => StatusCode::BAD_REQUEST,
        };
        (code, Json(self)).into_response()
    }
}
