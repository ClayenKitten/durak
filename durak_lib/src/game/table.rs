//! Container for cards that are being played.

use serde::{Deserialize, Serialize};

use super::card::{Card, CardSuit};

/// Table is the main location where the game is played.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct Table(Vec<(Card, Option<Card>)>);

impl Table {
    pub fn new() -> Self {
        Table(Vec::with_capacity(6))
    }

    pub fn count(&self) -> usize {
        debug_assert!(self.0.len() <= 6, "table shouldn't have more than 6 cards");
        self.0.len()
    }

    /// Places provided card as attacking.
    ///
    /// Returns `true` if card placed successfully.
    // TODO: allow attacking by multiple cards at once.
    pub fn attack(&mut self, card: Card) -> bool {
        if self.count() >= 6 || !self.all_attacks_answered() {
            return false;
        }
        for (attacking, defending) in self.0.iter() {
            if attacking.rank == card.rank {
                self.0.push((card, None));
                return true;
            }
            if defending.is_some_and(|def| def.rank == card.rank) {
                self.0.push((card, None));
                return true;
            }
        }
        false
    }

    /// Places provided card as defending.
    ///
    /// Returns `true` if card placed successfully.
    // TODO: allow specifying which card to defend against.
    pub fn defend(&mut self, card: Card, trump: CardSuit) -> bool {
        if self.all_attacks_answered() {
            return false;
        }
        let Some((attacking, defending @ None)) = self.0.last_mut() else {
            return false;
        };
        if !card.can_beat(*attacking, trump) {
            return false;
        }
        *defending = Some(card);
        true
    }

    /// Takes all cards from the table. Table is cleared.
    ///
    /// Returns all cards that were on the table.
    /// Returns `None` if taking cards is against rules.
    pub fn take(&mut self) -> Option<Vec<Card>> {
        if self.all_attacks_answered() {
            return None;
        }
        let cards = std::mem::replace(&mut self.0, Vec::with_capacity(6));
        let mut result = Vec::with_capacity(cards.len() * 2);
        for (attacking, defending) in cards {
            result.push(attacking);
            if let Some(defending) = defending {
                result.push(defending);
            }
        }
        Some(result)
    }

    /// Removes all cards from the table.
    ///
    /// Returns `true` if retreat was successful.
    pub fn retreat(&mut self) -> bool {
        if !self.all_attacks_answered() {
            return false;
        }
        self.0.clear();
        true
    }

    /// Returns `true` if all attacks on the table are answered.
    fn all_attacks_answered(&self) -> bool {
        self.0.iter().all(|(_, defending)| defending.is_some())
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
