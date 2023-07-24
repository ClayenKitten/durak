mod card;
mod setup;

use std::time::Duration;

use bevy::prelude::*;
use durak_lib::game::{card::CardSuit, hand::Hand, table::Table};

use crate::{
    network::{OnResponse, StatusRequest},
    session::Session,
    ui::game::display_ui,
    GameScreen,
};

/// Plugin that handles ongoing game management.
pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(setup::RoundSetupPlugin)
            .add_plugins(card::CardPlugin)
            .init_resource::<StatusRequestTimer>()
            .add_systems(
                Update,
                (request_status, on_status_response, display_ui)
                    .run_if(in_state(GameScreen::Round)),
            );
    }
}

fn request_status(
    mut timer: ResMut<StatusRequestTimer>,
    time: Res<Time>,
    session: Res<Session>,
    mut commands: Commands,
) {
    if timer.0.just_finished() {
        commands.spawn(StatusRequest(session.into_header()));
    }
    timer.0.tick(time.delta());
}

fn on_status_response(
    mut response: EventReader<OnResponse<StatusRequest>>,
    mut table: Query<&mut Table>,
    mut hand: Query<&mut Hand>,
) {
    let Some(OnResponse(status)) = response.iter().next() else {
        return;
    };

    let mut hand = hand.single_mut();
    *hand = status.hand.clone();

    let mut table = table.single_mut();
    *table = status.table.clone();
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

/// Trump suit for a round.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
