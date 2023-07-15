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

impl Card {
    /// Returns `true` if `self` can beat `other` (with regard to `trump`).
    pub fn can_beat(&self, other: Card, trump: CardSuit) -> bool {
        if self.suit == other.suit {
            self.rank > other.rank
        } else if self.suit == trump {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{CardRank, CardSuit};

    use super::Card;

    #[test]
    fn test_can_beat_common_suit() {
        let attacking = Card {
            suit: CardSuit::Clover,
            rank: CardRank::Jack,
        };

        let defending = Card {
            suit: CardSuit::Clover,
            rank: CardRank::Ace,
        };
        assert!(
            defending.can_beat(attacking, CardSuit::Diamond),
            "ace should be able to beat jack"
        );

        let defending = Card {
            suit: CardSuit::Clover,
            rank: CardRank::Six,
        };
        assert!(
            !defending.can_beat(attacking, CardSuit::Diamond),
            "six shouldn't be able to beat jack"
        );

        let defending = Card {
            suit: CardSuit::Clover,
            rank: CardRank::Jack,
        };
        assert!(
            !defending.can_beat(attacking, CardSuit::Diamond),
            "jack shouldn't be able to beat jack"
        );
    }

    #[test]
    fn test_can_beat_trump() {
        let attacking = Card {
            suit: CardSuit::Clover,
            rank: CardRank::Jack,
        };

        let defending = Card {
            suit: CardSuit::Diamond,
            rank: CardRank::Six,
        };
        assert!(
            defending.can_beat(attacking, CardSuit::Diamond),
            "trump should be able to beat any non-trump"
        );

        let defending = Card {
            suit: CardSuit::Diamond,
            rank: CardRank::Ace,
        };
        assert!(
            !defending.can_beat(attacking, CardSuit::Clover),
            "non-trump shouldn't be able to beat any trump"
        );
    }
}
