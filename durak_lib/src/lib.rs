#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use strum::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CardSuit {
    Clover,
    Diamond,
    Heart,
    Pike,
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
