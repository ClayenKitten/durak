pub mod cover;
pub mod interaction;
pub mod location;

use std::collections::HashMap;

use bevy::prelude::*;
use durak_lib::game::card::{Card, CardRank, CardSuit};
use strum::IntoEnumIterator;

/// Plugin that handles cards logic.
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_plugins(interaction::CardInteractionPlugin)
            .add_plugins(location::CardLocationPlugin)
            .add_plugins(cover::CardCoverPlugin);
    }
}

/// Creates entities for each possible card and stores mapping in [CardMapping].
fn setup(mut commands: Commands) {
    let mut mapping = HashMap::with_capacity(36);
    for suit in CardSuit::iter() {
        for rank in CardRank::iter() {
            let entity = commands.spawn(Card { suit, rank }).id();
            mapping.insert(Card { suit, rank }, entity);
        }
    }
    commands.insert_resource(CardMapping(mapping));
}

/// Storage that is used to map cards from its value to bevy's entity id.
// TODO: maybe use `[(Card, Entity); 36]` with custom lookup based on insertion order in `setup`
#[derive(Debug, Resource)]
pub struct CardMapping(HashMap<Card, Entity>);

impl CardMapping {
    pub fn get(&self, card: Card) -> Entity {
        *self
            .0
            .get(&card)
            .expect("CardMapping must contain every possible card")
    }
}

/// Set of static data associated with cards.
pub struct CardData;

impl CardData {
    pub const PIXEL_WIDTH: f32 = 42.;
    pub const PIXEL_HEIGHT: f32 = 60.;

    pub const SCALE: f32 = 3.;
    pub const WIDTH: f32 = Self::PIXEL_WIDTH * Self::SCALE;
    pub const HEIGHT: f32 = Self::PIXEL_HEIGHT * Self::SCALE;
    pub const SIZE: Vec2 = Vec2::new(Self::WIDTH, Self::HEIGHT);

    /// Sprite id for the back side of the card.
    pub const BACK_SPRITE_ID: usize = 27;

    pub fn sprite_atlas_id(card: Card) -> usize {
        use durak_lib::game::card::{CardRank::*, CardSuit::*};
        let row = match card.suit {
            Heart => 0,
            Diamond => 1,
            Clover => 2,
            Pike => 3,
        };
        let column = match card.rank {
            Ace => 0,
            Six => 5,
            Seven => 6,
            Eight => 7,
            Nine => 8,
            Ten => 9,
            Jack => 10,
            Queen => 11,
            King => 12,
        };
        row * 14 + column
    }
}
