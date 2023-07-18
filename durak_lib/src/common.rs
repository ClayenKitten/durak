//! Common data structures used by both server and client.

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{CardRank, CardSuit};

/// A unique identificator of game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameId(pub u64);

impl GameId {
    pub fn new(id: u64) -> Self {
        GameId(id)
    }
}

impl FromStr for GameId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let mut result = 0;
        for (index, char) in s.chars().rev().enumerate() {
            let Some(digit) = char.to_digit(16) else {
                return Err(());
            };
            result += digit as u64 * 16u64.pow(index as u32);
        }
        Ok(GameId(result))
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

/// A game-wide unique identificator of player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub fn new(id: u8) -> Self {
        PlayerId(id)
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    use std::str::FromStr;

    use crate::{common::GameId, CardRank, CardSuit};

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

    #[test]
    fn test_game_id_decoding() {
        let game_id = GameId::new(25);
        let s = game_id.to_string();
        let parsed = GameId::from_str(&s).unwrap();
        assert_eq!(game_id, parsed);
    }
}
