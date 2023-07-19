//! Systems that update location of cards.

use std::f32::consts::FRAC_PI_6;

use bevy::prelude::*;
use durak_lib::game::{card::Card, table::Table};

use crate::{card::CardData, collider::Collider, GameScreen, Hand};

use super::CardMapping;

/// Plugin that updates location of cards.
pub struct CardLocationPlugin;

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_hand_location, update_table_location).run_if(in_state(GameScreen::Round)),
        );
    }
}

/// Updates transform and visibility of each card that is contained in hand.
fn update_hand_location(
    mut commands: Commands,
    mapping: Res<CardMapping>,
    mut cards: Query<(&mut Transform, &mut Visibility), With<Card>>,
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
            let (mut transform, mut visibility) = cards.get_mut(entity).unwrap();
            transform.translation = Vec3::new(x, y, 0.0);
            *visibility = Visibility::Visible;
            commands.entity(entity).insert(collider);
        }
    }
}

/// Updates transform and visibility of each card that placed on the table.
fn update_table_location(
    mut commands: Commands,
    mapping: Res<CardMapping>,
    mut cards: Query<(&mut Transform, &mut Visibility), With<Card>>,
    table: Query<&Table, Changed<Table>>,
) {
    if let Ok(table) = table.get_single() {
        for (attacking, defending) in table.iter() {
            let x = card_x_location(table.position(attacking).unwrap(), table.count(), 40.);
            let y = 0.;

            let entity = mapping.get(attacking);
            let (mut transform, mut visibility) = cards.get_mut(entity).unwrap();
            transform.translation = Vec3::new(x, y, 0.0);
            *visibility = Visibility::Visible;
            commands.entity(entity).remove::<Collider>();

            if let Some(defending) = defending {
                let entity = mapping.get(defending);
                let (mut transform, mut visibility) = cards.get_mut(entity).unwrap();
                transform.translation = Vec3::new(x, y, 1.0);
                transform.rotation = Quat::from_rotation_z(-FRAC_PI_6);
                *visibility = Visibility::Visible;
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
