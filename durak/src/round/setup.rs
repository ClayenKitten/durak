//! Contains all data and logic used to setup new round.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use durak_lib::game::{deck::Deck, hand::Hand, table::Table};

use crate::{GameScreen, GameStarted};

use super::{
    card::{CardData, CardTextureAtlas},
    Trump,
};

pub struct RoundSetupPlugin;

impl Plugin for RoundSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (spawn_table, spawn_hand, spawn_deck.pipe(spawn_trump_card)),
                next_state,
            )
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
) -> Option<Entity> {
    if event_reader.is_empty() {
        return None;
    }

    let deck_position = Vec3 {
        x: camera.single().area.min.x + CardData::WIDTH / 2. + 16.,
        z: 1.,
        ..default()
    };

    let id = commands
        .spawn((
            Deck::new(),
            SpriteSheetBundle {
                transform: Transform::from_translation(deck_position)
                    .with_scale(Vec3::splat(CardData::SCALE)),
                texture_atlas: Handle::clone(&texture_atlas.0),
                sprite: TextureAtlasSprite::new(CardData::BACK_SPRITE_ID),
                ..default()
            },
        ))
        .id();

    Some(id)
}

pub fn spawn_trump_card(
    deck: In<Option<Entity>>,
    mut commands: Commands,
    texture_atlas: Res<CardTextureAtlas>,
    mut events: EventReader<GameStarted>,
) {
    let In(Some(deck)) = deck else { return; };
    if let Some(GameStarted { trump, .. }) = events.iter().next() {
        let trump = commands
            .spawn((
                Trump(trump.suit),
                SpriteSheetBundle {
                    texture_atlas: Handle::clone(&texture_atlas.0),
                    sprite: TextureAtlasSprite::new(CardData::sprite_atlas_id(*trump)),
                    transform: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))
                        .with_translation(Vec3::X * 20.),
                    ..default()
                },
            ))
            .id();
        commands.entity(deck).add_child(trump);
    }
}

fn spawn_hand(mut commands: Commands) {
    commands.spawn(Hand::default());
}

fn next_state(mut next: ResMut<NextState<GameScreen>>) {
    next.0 = Some(GameScreen::Round);
}
