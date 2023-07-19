mod card;
mod collider;
mod main_menu;
mod network;
mod round;
mod round_setup;
mod ui_utils;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_egui::EguiPlugin;
use card::CardPlugin;
use durak_lib::{
    game::{card::CardSuit, hand::Hand},
    identifiers::PlayerId,
};
use main_menu::MainMenuPlugin;
use network::NetworkPlugin;
use round::RoundPlugin;
use round_setup::RoundSetupPlugin;

fn main() {
    App::new()
        .add_event::<GameStarted>()
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
        .add_plugins(EguiPlugin)
        .add_state::<GameScreen>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(RoundSetupPlugin)
        .add_plugins(RoundPlugin)
        .add_plugins(CardPlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, startup)
        .run();
}

#[derive(Debug, Component)]
pub struct Player {
    pub _name: String,
    pub is_controlled: bool,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1280.,
                height: 800.,
            },
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameScreen {
    #[default]
    MainMenu,
    RoundSetup,
    Round,
}

#[derive(Debug, Event)]
pub struct GameStarted {
    /// Suit that is selected as trump for the game.
    pub trump: CardSuit,
    /// Players that play the game.
    pub players: Vec<PlayerId>,
}
