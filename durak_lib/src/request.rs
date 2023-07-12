//! Request data for both server and client to use.

#[cfg(feature = "serde")]
use bevy_ecs::event::Event;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::{CardRank, CardSuit};

/// Request that is sent by client and server to notify about card being played by player.
#[cfg_attr(feature = "bevy", derive(Event))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CardPlayed {
    pub player: usize,
    pub card: (CardRank, CardSuit),
}
