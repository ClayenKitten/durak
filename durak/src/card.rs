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
        app.init_resource::<CardTextureAtlas>()
            .add_systems(Startup, setup)
            .add_plugins(interaction::CardInteractionPlugin)
            .add_plugins(location::CardLocationPlugin)
            .add_plugins(cover::CardCoverPlugin);
    }
}

/// Creates entities for each possible card and stores mapping in [CardMapping].
fn setup(mut commands: Commands, atlas: Res<CardTextureAtlas>) {
    let mut mapping = HashMap::with_capacity(36);
    for suit in CardSuit::iter() {
        for rank in CardRank::iter() {
            let entity = commands
                .spawn((
                    Card { suit, rank },
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(CardData::sprite_atlas_id(Card {
                            suit,
                            rank,
                        })),
                        texture_atlas: Handle::clone(&atlas.0),
                        visibility: Visibility::Hidden,
                        transform: Transform::from_scale(Vec3::splat(CardData::SCALE)),
                        ..default()
                    },
                ))
                .id();
            mapping.insert(Card { suit, rank }, entity);
        }
    }
    commands.insert_resource(CardMapping(mapping));
}

#[derive(Debug, Clone, Resource)]
pub struct CardTextureAtlas(pub Handle<TextureAtlas>);

impl FromWorld for CardTextureAtlas {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let texture_handle = asset_server.load("cards.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(CardData::PIXEL_WIDTH, CardData::PIXEL_HEIGHT),
            14,
            4,
            None,
            None,
        );

        let mut texture_atlases = world.resource_mut::<Assets<TextureAtlas>>();
        let handle = texture_atlases.add(texture_atlas);

        CardTextureAtlas(handle)
    }
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
