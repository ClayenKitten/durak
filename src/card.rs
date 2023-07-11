use bevy::prelude::*;
use strum::EnumIter;

/// Set of static data associated with cards.
pub struct Card;

impl Card {
    pub const PIXEL_WIDTH: f32 = 42.;
    pub const PIXEL_HEIGHT: f32 = 60.;

    pub const SCALE: f32 = 3.;
    pub const WIDTH: f32 = Self::PIXEL_WIDTH * Self::SCALE;
    pub const HEIGHT: f32 = Self::PIXEL_HEIGHT * Self::SCALE;

    /// Sprite id for the back side of the card.
    pub const BACK_SPRITE_ID: usize = 27;

    pub fn sprite_atlas_id(suit: CardSuit, rank: CardRank) -> usize {
        let row = match suit {
            CardSuit::Heart => 0,
            CardSuit::Diamond => 1,
            CardSuit::Clover => 2,
            CardSuit::Pike => 3,
        };
        let column = match rank {
            CardRank::Ace => 0,
            CardRank::Six => 5,
            CardRank::Seven => 6,
            CardRank::Eight => 7,
            CardRank::Nine => 8,
            CardRank::Ten => 9,
            CardRank::Jack => 10,
            CardRank::Queen => 11,
            CardRank::King => 12,
        };
        row * 14 + column
    }
}

#[derive(Component, EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardSuit {
    Clover,
    Diamond,
    Heart,
    Pike,
}

#[derive(Component, EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

/// Marker component for card that is covered.
#[derive(Component, Debug, Clone, Copy)]
pub struct Covered;
