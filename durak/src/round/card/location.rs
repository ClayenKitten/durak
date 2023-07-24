//! Systems that update location of cards.

use std::f32::consts::FRAC_PI_6;

use bevy::prelude::*;
use durak_lib::game::{card::Card, table::Table};

use crate::{GameScreen, Hand};

use super::{collider::Collider, CardData, CardMapping};

/// Plugin that updates location of cards.
pub struct CardLocationPlugin;

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                card_visibility,
                (update_hand_location, update_table_location).before(card_visibility),
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
    hand: Query<&Hand, Changed<Hand>>,
    camera: Query<&OrthographicProjection>,
) {
    let area = camera.single().area;

    if let Ok(hand) = hand.get_single() {
        for card in hand.iter() {
            let x = card_x_location(hand.position(card).unwrap(), hand.count(), 10.);
            let y = area.min.y + CardData::HEIGHT / 2. - CardData::HEIGHT / 3.;
            let collider = Collider::new(CardData::SIZE);

            let entity = mapping.get(card);
            let mut transform = cards.get_mut(entity).unwrap();
            transform.translation = Vec3::new(x, y, 0.0);
            commands.entity(entity).insert(collider);
        }
    }
}

/// Updates transform of each card that placed on the table.
fn update_table_location(
    mut commands: Commands,
    mapping: Res<CardMapping>,
    mut cards: Query<&mut Transform, With<Card>>,
    table: Query<&Table, Changed<Table>>,
) {
    if let Ok(table) = table.get_single() {
        for (attacking, defending) in table.iter() {
            let x = card_x_location(table.position(attacking).unwrap(), table.count(), 40.);
            let y = 0.;

            let entity = mapping.get(attacking);
            let mut transform = cards.get_mut(entity).unwrap();
            transform.translation = Vec3::new(x, y, 0.0);
            commands.entity(entity).remove::<Collider>();

            if let Some(defending) = defending {
                let entity = mapping.get(defending);
                let mut transform = cards.get_mut(entity).unwrap();
                transform.translation = Vec3::new(x, y, 1.0);
                transform.rotation = Quat::from_rotation_z(-FRAC_PI_6);
                commands.entity(entity).remove::<Collider>();
            }
        }
    }
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
