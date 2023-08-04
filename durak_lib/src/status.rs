//! Status of the game.

mod lobby;
mod round;

pub use lobby::LobbyStatus;
pub use round::RoundStatus;

use serde::{Deserialize, Serialize};

use crate::{
    game::{card::Card, player::Opponent},
    identifiers::PlayerId,
};

/// General state of the game that is known to all players.
#[deprecated]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    /// Game hasn't started yet, players are connecting.
    ///
    /// It may be started by host's command if `can_start` is true.
    Lobby {
        players: Vec<LobbyPlayerData>,
        can_start: bool,
    },
    /// Game has started.
    Started { trump: Card, players: Vec<Opponent> },
    /// Game has ended.
    Completed {
        winner_id: PlayerId,
        winner_name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyPlayerData {
    pub id: PlayerId,
    pub name: String,
}
