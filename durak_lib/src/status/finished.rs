use serde::{Deserialize, Serialize};

use crate::identifiers::PlayerId;

use super::PlayerData;

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishedStatus {
    pub players: Vec<PlayerData>,
    pub winner: PlayerId,
}

impl FinishedStatus {
    pub fn winner_name(&self) -> &str {
        self.players
            .iter()
            .find(|data| data.id == self.winner)
            .map(|data| data.name.as_str())
            .expect("invalid finished status report")
    }
}
