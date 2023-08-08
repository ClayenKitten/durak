//! Status of the game.

pub mod lobby;
pub mod round;

use serde::{Deserialize, Serialize};

use crate::identifiers::PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyPlayerData {
    pub id: PlayerId,
    pub name: String,
}
