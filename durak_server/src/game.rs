//! Game state and logic.

use durak_lib::{
    game::{
        card::{Card, CardSuit},
        deck::Deck,
        hand::Hand,
        table::Table,
    },
    identifiers::PlayerId,
    network::JoinGameError,
    status::{GameState, GameStatus},
};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Game {
    pub host: PlayerId,
    pub password: String,
    state: GameState,
    trump: CardSuit,
    deck: Deck,
    players: Vec<Player>,
    round: Option<RoundState>,
}

impl Game {
    /// Creates new game with provided password set.
    pub fn new(password: String) -> Self {
        Self {
            host: PlayerId::new(0),
            password,
            trump: Self::pick_trump(),
            state: GameState::Lobby {
                players: vec![PlayerId::new(0)],
                can_start: false,
            },
            deck: Deck::new(),
            players: vec![Player {
                id: PlayerId::new(0),
                hand: Hand::default(),
            }],
            round: None,
        }
    }

    /// Returns current state of the game.
    pub fn state(&self) -> &GameState {
        &self.state
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
            hand: self
                .players
                .iter()
                .find(|p| p.id == player)
                .map(|player| player.hand.clone())?,
        })
    }

    /// Attempts to join existing game with id and password.
    ///
    /// Returns [PlayerId] if successful.
    pub fn join(&mut self, password: String) -> Result<PlayerId, JoinGameError> {
        if self.password != password {
            return Err(JoinGameError::InvalidPassword);
        }
        self.add_player().ok_or(JoinGameError::TooManyPlayers)
    }

    /// Adds new player to the game.
    ///
    /// Returns `None` if game is already full.
    /// Otherwise, returns [PlayerId] of the new player.
    pub fn add_player(&mut self) -> Option<PlayerId> {
        if self.players.len() > 1 {
            return None;
        }
        let id = PlayerId::new(self.players.len() as u8);
        self.players.push(Player {
            id,
            hand: Hand::default(),
        });
        if let GameState::Lobby { players, can_start } = &mut self.state {
            players.push(id);
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
    pub fn start(&mut self) {
        self.deck.shuffle();
        for player in self.players.iter_mut() {
            for _ in 0..6 {
                let card = self.deck.take().unwrap();
                player.hand.add(card)
            }
        }

        let mut players = self.players.iter();
        let attacker = players
            .next()
            .expect("at least two players are required to start the game")
            .id;
        let defender = players
            .next()
            .expect("at least two players are required to start the game")
            .id;

        self.round = Some(RoundState {
            table: Table::new(),
            attacker,
            defender,
        });
        // TODO: follow game's rules about first player.
        self.state = GameState::Started {
            trump: self.trump,
            players: self.players.iter().map(|p| p.id).collect(),
        };
    }

    /// Places card on the table.
    ///
    /// Returns `true` if played successfully.
    pub fn play_card(&mut self, player_id: PlayerId, card: Card) -> bool {
        let Some(ref mut round) = self.round else {
            return false;
        };
        let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) else {
            return false;
        };

        if round.attacker == player_id {
            player.hand.remove(card) && round.table.attack(card)
        } else if round.defender == player_id {
            player.hand.remove(card) && round.table.defend(card, self.trump)
        } else {
            false
        }
    }

    pub fn retreat(&mut self, player_id: PlayerId) -> bool {
        let Some(ref mut round) = self.round else {
            return false;
        };
        if round.attacker != player_id {
            return false;
        }
        if round.table.retreat() {
            round.swap_players();
            true
        } else {
            false
        }
    }

    pub fn take(&mut self, player_id: PlayerId) -> bool {
        let Some(ref mut round) = self.round else {
            return false;
        };
        let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) else {
            return false;
        };

        if round.defender != player_id {
            return false;
        }
        let Some(cards) = round.table.take() else {
            return false;
        };
        for card in cards {
            player.hand.add(card);
        }
        true
    }

    fn pick_trump() -> CardSuit {
        match thread_rng().gen_range(0..=3) {
            0 => CardSuit::Clover,
            1 => CardSuit::Diamond,
            2 => CardSuit::Heart,
            3 => CardSuit::Pike,
            _ => unreachable!(),
        }
    }
}

/// State of the game that is unique to the round.
#[derive(Debug)]
struct RoundState {
    pub table: Table,
    pub attacker: PlayerId,
    pub defender: PlayerId,
}

impl RoundState {
    /// Swaps attacker and defender.
    // TODO: allow more than two players.
    pub fn swap_players(&mut self) {
        std::mem::swap(&mut self.attacker, &mut self.defender)
    }

    /// Returns id of the player whose turn is it to play.
    pub fn turn(&self) -> PlayerId {
        if self.table.all_attacks_answered() {
            self.attacker
        } else {
            self.defender
        }
    }
}

#[derive(Debug)]
struct Player {
    pub id: PlayerId,
    pub hand: Hand,
}
