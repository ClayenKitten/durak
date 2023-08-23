use serde::{Deserialize, Serialize};

use super::PlayerData;

/// Status of the preparing game.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub struct LobbyStatus {
    /// List of joined players.
    pub players: Vec<PlayerData>,
}

impl LobbyStatus {
    /// Returns `true` if the game may be started by the host.
    pub fn can_start(&self) -> bool {
        // TODO: allow more than two players.
        self.players.len() == 2
    }
}
