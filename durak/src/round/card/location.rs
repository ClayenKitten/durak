//! Systems that update location of cards.

use std::f32::consts::FRAC_PI_6;

use bevy::prelude::*;
use durak_lib::game::{card::Card, player::Opponent, table::Table};

use crate::{
    network::{OnResponse, StatusRequest},
    GameScreen, Hand,
};

use super::{collider::Collider, CardData, CardMapping, CardTextureAtlas};

/// Plugin that updates location of cards.
pub struct CardLocationPlugin;

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                card_visibility,
                (update_hand_location, update_table_location).before(card_visibility),
                update_opponent_location,
            )
                .run_if(in_state(GameScreen::Round)),
        );
    }
}

/// Updates visibility of cards.
fn card_visibility(
    mut cards: Query<(&mut Visibility, &Card)>,
    hand: Query<&Hand>,
    table: Query<&Table>,
) {
    let hand = hand.single();
    let table = table.single();
    for (mut visibility, card) in cards.iter_mut() {
        if hand.contains(*card) || table.contains(*card) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Updates transform of each card that is contained in hand.
fn update_hand_location(
    mut commands: Commands,
    mapping: Res<CardMapping>,
    mut cards: Query<&mut Transform, With<Card>>,
    hand: Query<(Entity, &Hand), Changed<Hand>>,
) {
    let Ok((hand_entity, hand)) = hand.get_single() else {
        return;
    };

    commands.entity(hand_entity).clear_children();

    for card in hand.iter() {
        let x = card_x_location(hand.position(card).unwrap(), hand.count(), 10.);
        let y = 0.;
        let z = 0.;
        let angle = 0.;

        let collider = Collider::new(CardData::SIZE);

        let entity = mapping.get(card);
        let mut transform = cards.get_mut(entity).unwrap();
        transform.translation = Vec3 { x, y, z };
        transform.rotation = Quat::from_rotation_z(angle);
        commands.entity(entity).insert(collider);

        commands.entity(hand_entity).add_child(entity);
    }
}

/// Updates transform of each card that placed on the table.
fn update_table_location(
    mut commands: Commands,
    mapping: Res<CardMapping>,
    mut cards: Query<&mut Transform, With<Card>>,
    table: Query<(Entity, &Table), Changed<Table>>,
) {
    let Ok((table_entity, table)) = table.get_single() else {
        return;
    };

    let mut update = move |card: Card, is_defending: bool| {
        let x = card_x_location(table.position(card).unwrap(), table.count(), 40.);
        let y = 0.;
        let z = if is_defending { 1.0 } else { 0.0 };
        let angle = if is_defending { -FRAC_PI_6 } else { 0.0 };

        let card_entity = mapping.get(card);
        commands.entity(table_entity).add_child(card_entity);

        let mut transform = cards.get_mut(card_entity).unwrap();
        transform.translation = Vec3 { x, y, z };
        transform.rotation = Quat::from_rotation_z(angle);
        commands.entity(card_entity).remove::<Collider>();
    };

    for (attacking, defending) in table.iter() {
        update(attacking, false);
        if let Some(defending) = defending {
            update(defending, true);
        }
    }
}

#[derive(Component, Debug)]
struct OpponentCard;

/// Updates displayed opponent cards.
fn update_opponent_location(
    mut response: EventReader<OnResponse<StatusRequest>>,
    mut commands: Commands,
    opponents: Query<Entity, With<Opponent>>,
    camera: Query<&OrthographicProjection>,
    texture_atlas: Res<CardTextureAtlas>,
) {
    let Some(OnResponse(response)) = response.iter().next() else {
        return;
    };

    for opponent in opponents.iter() {
        commands.entity(opponent).despawn_recursive();
    }

    // TODO: allow more than one opponent.
    debug_assert!(response.opponents.len() == 1);
    let opponent = &response.opponents[0];

    let y = camera.single().area.max.y;

    let mut entity_commands = commands.spawn((
        opponent.clone(),
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., y, 0.)),
            ..default()
        },
    ));

    entity_commands.with_children(|parent| {
        for i in 0..opponent.cards_number {
            let x = card_x_location(i as usize, opponent.cards_number as usize, 10.);
            parent.spawn((
                OpponentCard,
                SpriteSheetBundle {
                    transform: Transform::from_translation(Vec3::new(x, 0., 0.))
                        .with_scale(Vec3::splat(CardData::SCALE)),
                    texture_atlas: Handle::clone(&texture_atlas.0),
                    sprite: TextureAtlasSprite::new(CardData::BACK_SPRITE_ID),
                    ..default()
                },
            ));
        }
    });
}

/// Calculates horizontal coordinate for card in player's hand or on the table.
fn card_x_location(index: usize, total: usize, gap: f32) -> f32 {
    debug_assert!(index < total);

    let max_offset = {
        let number_of_cards = (total - 1) as f32;
        number_of_cards * CardData::WIDTH + number_of_cards * gap
    };
    let x = {
        let number = index as f32;
        let offset = number * CardData::WIDTH + number * gap;
        offset - max_offset / 2.
    };
    x
}
