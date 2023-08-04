//! Game state and logic.

pub mod finished;
pub mod lobby;
pub mod round;

use self::{finished::FinishedState, lobby::LobbyState, round::RoundState};

#[derive(Debug)]
pub struct Game {
    phase: GamePhase,
}

impl Game {
    /// Creates new game with provided password set.
    pub fn new() -> Self {
        Self {
            phase: GamePhase::new(),
        }
    }

    /// Returns lobby state of the game.
    pub fn lobby_state(&mut self) -> Option<&mut LobbyState> {
        if let GamePhase::Lobby(state) = &mut self.phase {
            Some(state)
        } else {
            None
        }
    }

    /// Returns round state of the game.
    pub fn round_state(&mut self) -> Option<&mut RoundState> {
        if let GamePhase::Round(state) = &mut self.phase {
            Some(state)
        } else {
            None
        }
    }

    /// Returns finished state of the game.
    pub fn finished_state(&mut self) -> Option<&mut FinishedState> {
        if let GamePhase::Finished(state) = &mut self.phase {
            Some(state)
        } else {
            None
        }
    }

    /// Starts the game.
    ///
    /// Deck is shuffled and cards are dealed.
    ///
    /// Returns `true` if started successfully.
    pub fn start(&mut self) -> bool {
        if let GamePhase::Lobby(state) = &mut self.phase {
            self.phase = GamePhase::Round(state.to_started());
            true
        } else {
            false
        }
    }
}

/// Phase-specific state of the game.
#[derive(Debug)]
enum GamePhase {
    /// Preparation phase of the game.
    Lobby(LobbyState),
    /// Main phase of the game.
    Round(RoundState),
    /// Game is finished and winner is selected.
    Finished(FinishedState),
}

impl GamePhase {
    pub fn new() -> Self {
        GamePhase::Lobby(LobbyState::new())
    }
}
