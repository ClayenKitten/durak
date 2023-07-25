//! Components that define data associated with specific players.

use serde::{Deserialize, Serialize};

use crate::{game::hand::Hand, identifiers::PlayerId};

/// Full information about specific player.
///
/// `Player` is known to client associated with it and to the server.
/// Other players only have part of it information, see [Opponent].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub hand: Hand,
}

/// Limited information about specific player.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct Opponent {
    pub id: PlayerId,
    pub name: String,
    pub cards_number: u8,
}

impl From<Player> for Opponent {
    fn from(player: Player) -> Self {
        Self {
            id: player.id,
            name: player.name,
            cards_number: player.hand.count() as u8,
        }
    }
}
