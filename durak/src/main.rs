mod card;
mod collider;
mod main_menu;
mod network;
mod round;
mod round_setup;

use bevy::{prelude::*, render::camera::ScalingMode};
use card::CardInteractionPlugin;
use main_menu::MainMenuPlugin;
use network::NetworkPlugin;
use round_setup::RoundSetupPlugin;

#[derive(Debug, Component)]
pub struct Player {
    pub _name: String,
    pub is_controlled: bool,
}

/// Hand of the player containing all cards the player has.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand(Vec<Entity>);

impl Hand {
    /// Adds card into the hand.
    pub fn add(&mut self, card: Entity) {
        self.0.push(card);
    }

    /// Returns number of cards in the hand.
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if hand is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Checks if card is in the hand.
    pub fn contains(&self, card: Entity) -> bool {
        self.0.contains(&card)
    }

    /// Removes card from hand.
    ///
    /// # Panics
    ///
    /// Panics if card is not in the hand.
    pub fn remove(&mut self, card: Entity) {
        let Some(index) = self.0.iter().position(|e| *e == card) else {
            panic!(
                "attempted to remove card `{card:?}` that is not in the hand. \
                Cards in the hand are: {:?}",
                self.0.as_slice(),
            );
        };
        self.0.remove(index);
    }
}

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
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameScreen>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(RoundSetupPlugin)
        .add_plugins(CardInteractionPlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<GameScreen>>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 1280., height: 800. },
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
    commands.spawn((
        Player { _name: String::from("Me"), is_controlled: true },
        Hand::default(),
    ));
    commands.spawn((
        Player { _name: String::from("Not me"), is_controlled: false },
        Hand::default(),
    ));
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameScreen {
    #[default]
    MainMenu,
    RoundSetup,
    Round,
}
