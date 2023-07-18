use serde::{Deserialize, Serialize};

use strum::EnumIter;

/// A single card.
///
/// Each deck must have exactly one card of each type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
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

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardSuit {
    Clover,
    Diamond,
    Heart,
    Pike,
}

#[derive(
    EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum CardRank {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[cfg(test)]
mod test {
    use crate::game::card::{CardRank, CardSuit};

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
