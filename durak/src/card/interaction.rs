//! Systems that handle user interaction with cards.

use bevy::prelude::*;
use durak_lib::game::{card::Card, hand::Hand, table::Table};

use crate::{collider, GameScreen, Player};

pub struct CardInteractionPlugin;

impl Plugin for CardInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardClicked>()
            .add_event::<CardHoverStarted>()
            .add_event::<CardHoverEnded>()
            .add_systems(
                Update,
                (collider::cursor_system, card_click)
                    .chain()
                    .run_if(in_state(GameScreen::Round)),
            );
    }
}

/// Handles clicks on cards.
fn card_click(
    mut event_reader: EventReader<CardClicked>,
    mut player: Query<(&Player, &mut Hand)>,
    cards: Query<&Card>,
    mut table: Query<&mut Table>,
) {
    let mut table = table.single_mut();
    for CardClicked(entity) in event_reader.iter() {
        let card = *cards.get(*entity).unwrap();
        for (player, mut hand) in player.iter_mut() {
            if player.is_controlled && hand.contains(card) {
                hand.remove(card);
                // TODO: either attack or defend.
                table.attack(card);
                break;
            }
        }
    }
}

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct CardClicked(pub Entity);

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct CardHoverStarted(pub Entity);

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct CardHoverEnded(pub Entity);
