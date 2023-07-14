//! Game state and logic.

use durak_lib::{
    common::{Card, PlayerId},
    network::JoinGameError,
    CardRank, CardSuit,
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Game {
    pub password: String,
    trump: CardSuit,
    state: GameState,
    deck: Deck,
    table: Vec<(Card, Option<Card>)>,
    players: Vec<Player>,
}

impl Game {
    /// Creates new game with provided password set.
    pub fn new(password: String) -> Self {
        Self {
            password,
            trump: Self::pick_trump(),
            state: GameState::Created,
            deck: Deck::new(),
            table: Vec::with_capacity(6),
            players: vec![Player {
                id: PlayerId::new(0),
                hand: Hand::default(),
            }],
        }
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
        Some(id)
    }

    fn pick_trump() -> CardSuit {
        match rand::thread_rng().gen_range(0..=3) {
            0 => CardSuit::Clover,
            1 => CardSuit::Diamond,
            2 => CardSuit::Heart,
            3 => CardSuit::Pike,
            _ => unreachable!(),
        }
    }
}

/// State of the game.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameState {
    /// Game is created, but not enough players connected.
    Created,
    /// Game is ready to start on host's command.
    ReadyToStart,
    /// Expecting specified player's action.
    ExpectAction(PlayerId),
    /// Game is ended.
    Completed { win: PlayerId },
}

#[derive(Debug)]
struct Deck(Vec<Card>);

impl Deck {
    /// Creates new (not shuffled) deck.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(36);
        for suit in CardSuit::iter() {
            for rank in CardRank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        Self(cards)
    }

    /// Shuffles all cards in the deck.
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    /// Takes card from the top of the deck.
    pub fn take(&mut self) -> Option<Card> {
        self.0.pop()
    }

    /// Returns `true` if deck is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug)]
struct Player {
    pub id: PlayerId,
    pub hand: Hand,
}

#[derive(Debug, Default)]
struct Hand(pub Vec<Card>);
