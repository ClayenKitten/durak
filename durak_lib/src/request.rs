//! Request data for both server and client to use.

#[cfg(feature = "bevy")]
use bevy_ecs::event::Event;
use serde::{Deserialize, Serialize};

use crate::{CardRank, CardSuit};

/// Request that is sent by client and server to notify about card being played by player.
#[cfg_attr(feature = "bevy", derive(Event))]
#[derive(Serialize, Deserialize)]
pub struct CardPlayed {
    pub player: usize,
    pub card: (CardRank, CardSuit),
}
