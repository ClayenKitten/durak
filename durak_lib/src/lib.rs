pub mod request;

#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;

use serde::{Deserialize, Serialize};

use strum::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub enum CardSuit {
    Clover,
    Diamond,
    Heart,
    Pike,
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component))]
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
