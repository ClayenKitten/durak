//! Contains all data and logic used to setup new round.

use bevy::prelude::*;
use durak_lib::game::{deck::Deck, hand::Hand, table::Table};

use crate::{
    card::{CardData, CardTextureAtlas},
    GameScreen, GameStarted,
};

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
    texture_atlas: Res<CardTextureAtlas>,
    camera: Query<&OrthographicProjection>,
) {
    if event_reader.is_empty() {
        return;
    }

    let deck_position = Vec3 {
        x: camera.single().area.min.x + CardData::WIDTH / 2. + 16.,
        z: 1.,
        ..default()
    };

    commands.spawn((
        Deck::new(),
        SpriteSheetBundle {
            transform: Transform::from_translation(deck_position),
            texture_atlas: Handle::clone(&texture_atlas.0),
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
