//! Contains all data and logic used to setup new round.

use bevy::prelude::*;
use durak_lib::game::{hand::Hand, table::Table};

use crate::{GameScreen, GameStarted};

pub struct RoundSetupPlugin;

impl Plugin for RoundSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((spawn_table, spawn_hand), next_state)
                .chain()
                .run_if(in_state(GameScreen::RoundSetup)),
        );
    }
}

fn spawn_table(event_reader: EventReader<GameStarted>, mut commands: Commands) {
    if event_reader.is_empty() {
        return;
    }
    commands.spawn(Table::default());
}

fn spawn_hand(mut commands: Commands) {
    commands.spawn(Hand::default());
}

fn next_state(mut next: ResMut<NextState<GameScreen>>) {
    next.0 = Some(GameScreen::Round);
}
