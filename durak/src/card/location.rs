//! Systems that update location of cards.

use bevy::prelude::*;
use durak_lib::game::{card::Card, table::Table};

use crate::{card::CardData, collider::Collider, GameScreen, Hand};

/// Plugin that updates location of cards.
pub struct CardLocationPlugin;

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_card_location.run_if(in_state(GameScreen::Round)),
        );
    }
}

/// Marker component for card that is moved to table.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// Card's exact location is unknown: it is either in the deck or in another player's hand.
    Unknown,
    /// Card in placed on the table.
    Table,
    /// Card is moved to dicard pile.
    Discarded,
    /// Card is kept in current player's hand.
    PlayerHand,
}

/// Updates transform of each card whose [Location] was changed.
fn update_card_location(
    mut commands: Commands,
    mut cards: Query<
        (Entity, &mut Transform, &mut Visibility, &Location, &Card),
        Changed<Location>,
    >,
    table: Query<&Table>,
    hand: Query<&Hand>,
    camera: Query<&OrthographicProjection>,
) {
    let area = camera.single().area;
    let table = table.single();
    let hand = hand.single();
    for (entity, mut transform, mut visibility, location, &card) in cards.iter_mut() {
        use Location::*;
        *visibility = match location {
            Unknown => Visibility::Hidden,
            Table | Discarded | PlayerHand => Visibility::Visible,
        };
        let y = match location {
            Unknown | Table | Discarded => 0.,
            PlayerHand => area.min.y + CardData::HEIGHT / 2. - CardData::HEIGHT / 3.,
        };
        let x = match location {
            Unknown => area.min.x + CardData::WIDTH / 2. + 16.,
            Discarded => area.max.x - CardData::WIDTH,
            Table => card_x_location(table.position(card).unwrap(), table.count(), 40.),
            PlayerHand => card_x_location(hand.position(card).unwrap(), hand.count(), 10.),
        };
        let collider = Collider::new(CardData::SIZE);
        transform.translation = Vec3::new(x, y, 0.0);
        commands.entity(entity).insert(collider);
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
