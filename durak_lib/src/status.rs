/// Status of the game.
use serde::{Deserialize, Serialize};

use crate::{
    game::{card::Card, hand::Hand, table::Table},
    identifiers::PlayerId,
};

/// General state of the game that is known to all players.
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
    Started { trump: Card, players: Vec<PlayerId> },
    /// Game has ended.
    Completed { win: PlayerId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyPlayerData {
    pub id: PlayerId,
    pub name: String,
}

/// Status of the ongoing game that is known to specific player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStatus {
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
}
