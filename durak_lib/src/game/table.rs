//! Container for cards that are being played.

use serde::{Deserialize, Serialize};

use crate::errors::PlayCardError;

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
    // TODO: allow attacking by multiple cards at once.
    pub fn attack(&mut self, card: Card) -> Result<(), PlayCardError> {
        if self.count() >= 6 {
            return Err(PlayCardError::TooMuchAttackingCards);
        }
        if !self.all_attacks_answered() {
            return Err(PlayCardError::InvalidTurn);
        }
        if self.count() == 0 {
            self.0.push((card, None));
            return Ok(());
        }
        for (attacking, defending) in self.0.iter() {
            if attacking.rank == card.rank {
                self.0.push((card, None));
                return Ok(());
            }
            if defending.is_some_and(|def| def.rank == card.rank) {
                self.0.push((card, None));
                return Ok(());
            }
        }
        Err(PlayCardError::CantPlace)
    }

    /// Places provided card as defending.
    // TODO: allow specifying which card to defend against.
    pub fn defend(&mut self, card: Card, trump: CardSuit) -> Result<(), PlayCardError> {
        if self.all_attacks_answered() {
            return Err(PlayCardError::InvalidTurn);
        }
        let Some((attacking, defending @ None)) = self.0.last_mut() else {
            return Err(PlayCardError::InvalidTurn);
        };
        if !card.can_beat(*attacking, trump) {
            return Err(PlayCardError::CantPlace);
        }
        *defending = Some(card);
        Ok(())
    }

    /// Takes all cards from the table. Table is cleared.
    ///
    /// Returns all cards that were on the table.
    /// Returns `None` if taking cards is against the rules.
    pub fn take(&mut self) -> Option<Vec<Card>> {
        if self.can_take() {
            let cards = std::mem::replace(&mut self.0, Vec::with_capacity(6));
            let mut result = Vec::with_capacity(cards.len() * 2);
            for (attacking, defending) in cards {
                result.push(attacking);
                if let Some(defending) = defending {
                    result.push(defending);
                }
            }
            Some(result)
        } else {
            None
        }
    }

    /// Removes all cards from the table.
    ///
    /// Returns `true` if retreat was successful.
    pub fn retreat(&mut self) -> bool {
        if self.can_retreat() {
            self.0.clear();
            true
        } else {
            false
        }
    }

    /// Returns `true` if all attacks on the table are answered.
    pub fn all_attacks_answered(&self) -> bool {
        self.0.iter().all(|(_, defending)| defending.is_some())
    }

    /// Returns number of slots that have cards in them.
    pub fn occupied_slots(&self) -> usize {
        self.0
            .iter()
            .map(|(_, def)| if def.is_some() { 2 } else { 1 })
            .sum()
    }

    /// Returns iterator over slots of table.
    pub fn iter(&self) -> impl Iterator<Item = (Card, Option<Card>)> + '_ {
        self.0.iter().cloned()
    }

    /// Returns position of the card on the table.
    pub fn position(&self, card: Card) -> Option<usize> {
        self.0.iter().position(|c| c.0 == card || c.1 == Some(card))
    }

    /// Returns `true` if table contains the card.
    pub fn contains(&self, card: Card) -> bool {
        self.0.iter().any(|c| c.0 == card || c.1 == Some(card))
    }

    /// Returns `true` if table is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Methods that check if action is allowed.
impl Table {
    /// Returns `true` if the attacker can retreat.
    pub fn can_retreat(&self) -> bool {
        self.all_attacks_answered() && !self.is_empty()
    }

    /// Returns `true` if the defender can take cards.
    pub fn can_take(&self) -> bool {
        !self.all_attacks_answered()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
