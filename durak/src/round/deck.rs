use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{GameScreen, GameStarted};

use super::{
    card::{CardData, CardTextureAtlas},
    Trump,
};

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScreen::RoundSetup),
            spawn_deck.pipe(spawn_trump_card),
        )
        .add_systems(Update, deck_visibility.run_if(in_state(GameScreen::Round)));
    }
}

#[derive(Component, Debug)]
pub struct Deck {
    pub left: u8,
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
            Deck { left: 36 },
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

fn spawn_trump_card(
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
                    visibility: Visibility::Visible,
                    ..default()
                },
            ))
            .id();
        commands.entity(deck).add_child(trump);
    }
}

fn deck_visibility(
    mut deck: Query<(&Deck, &mut Visibility), Without<Trump>>,
    mut trump: Query<&mut Visibility, With<Trump>>,
) {
    let (deck, mut deck_visibility) = deck.single_mut();
    if deck.left <= 1 {
        *deck_visibility = Visibility::Hidden;
        if deck.left == 0 {
            let mut trump_visibility = trump.single_mut();
            *trump_visibility = Visibility::Hidden;
        }
    }
}
