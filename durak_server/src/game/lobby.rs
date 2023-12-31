use durak_lib::{
    identifiers::PlayerId,
    status::{lobby::LobbyStatus, PlayerData},
};

use super::round::RoundState;

#[derive(Debug)]
pub struct LobbyState {
    players: Vec<PlayerData>,
}

impl LobbyState {
    /// Creates new lobby state.
    pub(super) fn new() -> Self {
        Self {
            players: Vec::with_capacity(4),
        }
    }

    /// Generates status report.
    pub fn status(&self) -> LobbyStatus {
        LobbyStatus {
            players: self.players.clone(),
        }
    }

    /// Adds new player to the game.
    ///
    /// Returns `None` if game is already full.
    /// Otherwise, returns [PlayerId] of the new player.
    pub fn add_player(&mut self, name: String) -> Option<PlayerId> {
        if self.players.len() > 1 {
            return None;
        }
        let id = PlayerId::new(self.players.len() as u8);
        self.players.push(PlayerData { id, name });
        Some(id)
    }

    /// Removes player from the game.
    ///
    /// Returns `true` if removed successfully.
    pub fn remove_player(&mut self, player_id: PlayerId) -> bool {
        // TODO: check if host leaves
        let Some(index) = self.players.iter().position(|p| p.id == player_id) else {
            return false;
        };
        self.players.remove(index);
        true
    }

    /// Returns `true` if the game may be started by the host.
    pub fn can_start(&self) -> bool {
        // TODO: allow more than two players.
        self.players.len() == 2
    }

    /// Transforms state to the [RoundState].
    pub fn to_started(&self) -> RoundState {
        RoundState::new(self.players.clone())
    }
}
