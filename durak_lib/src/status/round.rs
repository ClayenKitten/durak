use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    errors::AccessError,
    game::{hand::Hand, player::Opponent, table::Table},
    identifiers::PlayerId,
};

/// Status of the ongoing game that is known to specific player.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub struct RoundStatus {
    /// What player should take action now.
    pub turn: PlayerId,
    /// Attacker of the round.
    pub attacker: PlayerId,
    /// Defender of the round.
    pub defender: PlayerId,
    /// Table state.
    pub table: Table,
    /// Hand of the player.
    pub hand: Hand,
    /// Number of cards left in deck.
    pub deck_size: u8,
    /// List of opposing players.
    ///
    /// Doesn't include player that requested status report.
    pub opponents: Vec<Opponent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoundStatusResponse {
    Ok(RoundStatus),
    /// Game hasn't started yet.
    NotStarted,
    /// Failed to access some data.
    AccessError(AccessError),
}

impl From<AccessError> for RoundStatusResponse {
    fn from(value: AccessError) -> Self {
        RoundStatusResponse::AccessError(value)
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for RoundStatusResponse {
    fn into_response(self) -> Response {
        let code = match self {
            RoundStatusResponse::Ok(_) => StatusCode::OK,
            RoundStatusResponse::NotStarted => StatusCode::BAD_REQUEST,
            RoundStatusResponse::AccessError(_) => StatusCode::BAD_REQUEST,
        };
        (code, Json(self)).into_response()
    }
}
