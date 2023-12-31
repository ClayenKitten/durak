use serde::{Deserialize, Serialize};

use crate::{
    game::{card::Card, hand::Hand, player::Opponent, table::Table},
    identifiers::PlayerId,
};

/// Status of the ongoing game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundStatus {
    /// Trump for the round.
    pub trump: Card,
    /// What player should take action now.
    pub turn: PlayerId,
    /// Attacker of the round.
    pub attacker: PlayerId,
    /// Defender of the round.
    pub defender: PlayerId,
    /// Table state.
    pub table: Table,
    /// Hand of the player.
    pub hand: Hand,
    /// Number of cards left in deck.
    pub deck_size: u8,
    /// List of opposing players.
    ///
    /// Doesn't include player that requested status report.
    pub opponents: Vec<Opponent>,
}
