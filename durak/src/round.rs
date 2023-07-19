use std::time::Duration;

use bevy::prelude::*;
use durak_lib::{game::card::Card, network::AuthHeader};

use crate::{
    card::{location::Location, CardMapping},
    network::{OnResponce, StatusRequest},
    GameScreen,
};

/// Plugin that handles ongoing game management.
pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StatusRequestTimer>().add_systems(
            Update,
            (request_status, on_status_response).run_if(in_state(GameScreen::Round)),
        );
    }
}

fn request_status(
    mut timer: ResMut<StatusRequestTimer>,
    time: Res<Time>,
    auth: Res<AuthHeader>,
    mut commands: Commands,
) {
    if timer.0.just_finished() {
        commands.spawn(StatusRequest(auth.clone()));
    }
    timer.0.tick(time.delta());
}

fn on_status_response(
    mut response: EventReader<OnResponce<StatusRequest>>,
    mut commands: Commands,
    cards: Query<Entity, (With<Card>, With<Location>)>,
    mapping: Res<CardMapping>,
) {
    let Some(OnResponce(status)) = response.iter().next() else {
        return;
    };
    for card in cards.iter() {
        commands.entity(card).remove::<Location>();
    }
    for card in status.hand.iter() {
        commands
            .entity(mapping.get(card))
            .insert(Location::PlayerHand);
    }
    for card in status.table.iter() {
        commands.entity(mapping.get(card.0)).insert(Location::Table);
        if let Some(card) = card.1 {
            commands.entity(mapping.get(card)).insert(Location::Table);
        }
    }
}

#[derive(Debug, Resource)]
pub struct StatusRequestTimer(Timer);

impl Default for StatusRequestTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        timer.tick(Duration::from_secs_f32(0.5));
        Self(timer)
    }
}
