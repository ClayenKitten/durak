//! Systems that handle user interaction with cards.

use bevy::prelude::*;
use durak_lib::{network::AuthHeader, game::{hand::Hand, card::Card}};

use crate::{collider, GameScreen, network::PlayCardRequest};

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
pub fn card_click(
    mut commands: Commands,
    mut event_reader: EventReader<CardClicked>,
    mut hand: Query<&mut Hand>,
    auth: Res<AuthHeader>,
    cards: Query<&Card>,
) {
    let hand = hand.single_mut();
    for CardClicked(entity) in event_reader.iter() {
        let card = *cards.get(*entity).unwrap();
        if hand.contains(card) {
            commands.spawn(PlayCardRequest {
                auth: auth.clone(),
                card,
            });
        }
    }
}

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct CardClicked(pub Entity);

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct CardHoverStarted(pub Entity);

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct CardHoverEnded(pub Entity);
