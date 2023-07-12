mod card;
mod collider;
mod round;
mod round_setup;

use bevy::prelude::*;
use card::{CardInteractionPlugin, CardSuit};
use round_setup::RoundSetupPlugin;

#[derive(Debug, Component)]
pub struct Player {
    pub _name: String,
    pub is_controlled: bool,
}

/// Hand of the player containing all cards the player has.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand(Vec<Entity>);

/// List of cards that are still in deck.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Deck(Vec<Entity>);

/// Marker component for card that is discarded.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Discarded;

// Our Bevy app's entry point
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Durak"),
                        resolution: (1280., 800.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameScreen>()
        .add_plugins(RoundSetupPlugin)
        .add_plugins(CardInteractionPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, mut state: ResMut<NextState<GameScreen>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player { _name: String::from("Me"), is_controlled: true },
        Hand::default(),
    ));
    commands.spawn((
        Player { _name: String::from("Not me"), is_controlled: false },
        Hand::default(),
    ));
    state.0 = Some(GameScreen::RoundSetup);
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameScreen {
    #[default]
    MainMenu,
    RoundSetup,
    Round,
}
