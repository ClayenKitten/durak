//! Player's hand.

use super::card::Card;

/// Hand is a place where all cards owned by player are kept.
#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct Hand(Vec<Card>);

impl Hand {
    /// Creates new empty hand.
    pub fn new() -> Self {
        Hand(Vec::with_capacity(6))
    }

    /// Adds card to the hand.
    pub fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Removes card from the hand.
    ///
    /// Returns `true` if card was in the hand.
    pub fn remove(&mut self, card: Card) -> bool {
        match self.0.iter().position(|card_| *card_ == card) {
            Some(index) => {
                self.0.remove(index);
                true
            }
            None => false,
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}