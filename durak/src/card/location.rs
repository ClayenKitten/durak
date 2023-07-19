//! Systems that update location of cards.

use bevy::prelude::*;
use durak_lib::game::{card::Card, table::Table};

use crate::{card::CardData, GameScreen, Hand, Player};

/// Plugin that updates location of cards.
pub struct CardLocationPlugin;

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_to_hand, move_to_table, move_to_discard).run_if(in_state(GameScreen::Round)),
        );
    }
}

pub fn move_to_hand(
    mut _commands: Commands,
    mut _cards: Query<&mut Transform, With<Card>>,
    _hands: Query<(&Player, &Hand), Changed<Hand>>,
    _camera: Query<&OrthographicProjection>,
) {
}

fn move_to_table(
    mut _commands: Commands,
    mut _table: Query<&mut Table, Changed<Table>>,
    mut _cards: Query<&mut Transform, With<Card>>,
) {
}

/// Calculates horizontal coordinate for card in hand or on table.
fn _card_x_location(index: usize, total: usize, gap: f32) -> f32 {
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

fn move_to_discard() {}
