//! Common data structures used by both server and client.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{CardRank, CardSuit};

/// A unique identificator of game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameId(u64);

impl GameId {
    pub fn new(id: u64) -> Self {
        GameId(id)
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A game-wide unique identificator of player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerId(u8);

impl PlayerId {
    pub fn new(id: u8) -> Self {
        PlayerId(id)
    }
}

/// Single card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub suit: CardSuit,
    pub rank: CardRank,
}
