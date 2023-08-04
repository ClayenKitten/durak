//! Game state and logic.

pub mod finished;
pub mod lobby;
pub mod round;

use durak_lib::{
    game::{hand::Hand, player::Player},
    identifiers::PlayerId,
    status::{GameState, GameStatus, LobbyPlayerData},
};

use self::{finished::FinishedState, lobby::LobbyState, round::RoundState};

#[derive(Debug)]
pub struct Game {
    state: GameState,
    players: Vec<Player>,
    round: Option<RoundState>,
}

impl Game {
    /// Creates new game with provided password set.
    pub fn new() -> Self {
        Self {
            state: GameState::Lobby {
                players: Vec::new(),
                can_start: false,
            },
            players: Vec::new(),
            round: None,
        }
    }

    /// Returns current state of the game.
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Returns round state of the game.
    pub fn round(&mut self) -> Option<&mut RoundState> {
        self.round.as_mut()
    }

    /// Generates status report for specific player.
    pub fn status(&self, player: PlayerId) -> Option<GameStatus> {
        let Some(ref round) = self.round else {
            return None;
        };
        Some(GameStatus {
            turn: round.turn(),
            attacker: round.attacker,
            defender: round.defender,
            table: round.table.clone(),
            deck_size: round.deck.count() as u8,
            hand: self
                .players
                .iter()
                .find(|p| p.id == player)
                .map(|player| player.hand.clone())?,
            opponents: self
                .players
                .iter()
                .filter(|p| p.id != player)
                .cloned()
                .map(|player| player.into())
                .collect(),
        })
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
        self.players.push(Player {
            id,
            name: name.clone(),
            hand: Hand::default(),
        });
        if let GameState::Lobby { players, can_start } = &mut self.state {
            players.push(LobbyPlayerData { id, name });
            *can_start = self.players.len() >= 2;
        }
        Some(id)
    }

    /// Removes player from the game.
    ///
    /// Returns `true` if removed successfully.
    pub fn remove_player(&mut self, player_id: PlayerId) -> bool {
        let Some(index) = self.players.iter().position(|p| p.id == player_id) else {
            return false;
        };
        self.players.remove(index);
        if let GameState::Lobby { players, can_start } = &mut self.state {
            players.remove(index);
            *can_start = self.players.len() >= 2;
        }
        true
    }

    /// Starts the game.
    ///
    /// Deck is shuffled and cards are dealed.
    ///
    /// Returns `true` if started successfully.
    pub fn start(&mut self) -> bool {
        match self.state {
            GameState::Started { .. } => return false,
            GameState::Completed { .. } => return false,
            _ => {}
        }

        let round = RoundState::new(self.players.iter().map(|p| p.id).collect());
        // TODO: follow game's rules about first player.
        self.state = GameState::Started {
            trump: round.trump,
            players: self.players.iter().cloned().map(|p| p.into()).collect(),
        };
        self.round = Some(round);

        true
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
