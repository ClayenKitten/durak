//! Deck of cards used in game.

#![allow(clippy::new_without_default)]

use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

use super::card::{Card, CardRank, CardSuit};

/// Deck of cards that is used as main source of cards throughout the game.
///
/// Deck in standard version of the game consists of 36 unique cards.
#[derive(Debug)]
pub struct Deck(Vec<Card>);

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

    /// Returns number of cards left in deck.
    pub fn count(&self) -> usize {
        self.0.len()
    }
}
