//! Contains all data and logic used to setup new round.

use bevy::prelude::*;
use durak_lib::game::{card::CardSuit, deck::Deck, hand::Hand, table::Table};

use crate::{card::CardData, GameScreen, GameStarted};

pub struct RoundSetupPlugin;

impl Plugin for RoundSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((spawn_table, spawn_deck, spawn_hand), next_state)
                .chain()
                .run_if(in_state(GameScreen::RoundSetup)),
        );
    }
}

fn spawn_table(event_reader: EventReader<GameStarted>, mut commands: Commands) {
    if event_reader.is_empty() {
        return;
    }
    commands.spawn(Table::default());
}

fn spawn_deck(
    event_reader: EventReader<GameStarted>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera: Query<&OrthographicProjection>,
) {
    if event_reader.is_empty() {
        return;
    }
    let texture_handle = asset_server.load("cards.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(CardData::PIXEL_WIDTH, CardData::PIXEL_HEIGHT),
        14,
        4,
        None,
        None,
    );
    let texture_atlas = texture_atlases.add(texture_atlas);

    let deck_position = Vec3 {
        x: camera.single().area.min.x + CardData::WIDTH / 2. + 16.,
        z: 1.,
        ..default()
    };

    commands.spawn((
        Deck::new(),
        SpriteSheetBundle {
            transform: Transform::from_translation(deck_position),
            texture_atlas,
            sprite: TextureAtlasSprite::new(CardData::BACK_SPRITE_ID),
            ..default()
        },
    ));
}

fn spawn_hand(mut commands: Commands) {
    commands.spawn(Hand::default());
}

fn next_state(mut next: ResMut<NextState<GameScreen>>) {
    next.0 = Some(GameScreen::Round);
}

/// Trump suit for a round.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
