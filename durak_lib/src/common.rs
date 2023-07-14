//! Common data structures used by both server and client.

use serde::{Deserialize, Serialize};

use crate::{CardRank, CardSuit};

/// A game-wide unique identificator of player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
