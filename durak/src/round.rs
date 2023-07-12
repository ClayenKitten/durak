use bevy::prelude::*;

use durak_lib::CardSuit;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Table([TableSlot; 6]);

impl Table {
    /// Plays card on the table.
    ///
    /// Returns `true` if played successfully.
    pub fn play(&mut self, card: Entity) -> bool {
        let slot = self
            .0
            .iter_mut()
            .find(|slot| slot.defending.is_none() || slot.attacking.is_none());

        let Some(slot) = slot else { return false; };
        if let Some(_attacking) = slot.attacking {
            // TODO: check that card can be beaten.
            slot.defending = Some(card);
        } else {
            slot.attacking = Some(card);
        }
        true
    }

    /// Returns number of slots that have cards in them.
    pub fn occupied_slots(&self) -> usize {
        self.0.iter().filter(|slot| !slot.is_empty()).count()
    }

    pub fn slots(&self) -> &[TableSlot] {
        self.0.as_slice()
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TableSlot {
    pub attacking: Option<Entity>,
    pub defending: Option<Entity>,
}

impl TableSlot {
    pub fn is_empty(&self) -> bool {
        debug_assert!({
            if self.attacking.is_none() {
                self.defending.is_none()
            } else {
                true
            }
        });
        self.attacking.is_none()
    }
}

/// Trump suit for a round.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
