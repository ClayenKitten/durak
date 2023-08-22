#![windows_subsystem = "windows"]

mod network;
mod persistence;
mod round;
mod session;
mod ui;

use bevy::{prelude::*, render::camera::ScalingMode};
use durak_lib::{
    game::{card::Card, hand::Hand, player::Opponent},
    identifiers::PlayerId,
};

use network::NetworkPlugin;
use persistence::Configuration;
use round::RoundPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_event::<GameStarted>()
        .add_event::<GameEnded>()
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
        .add_plugins(UiPlugin)
        .add_plugins(RoundPlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    match Configuration::load() {
        Ok(configuration) => {
            commands.insert_resource(configuration);
        }
        Err(_) => {
            let configuration = Configuration::default();
            let _ = Configuration::save(&configuration);
            commands.insert_resource(configuration);
        }
    }

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
    Round,
}

/// Event that is sent when the game is started.
#[derive(Debug, Event)]
pub struct GameStarted {
    /// Suit that is selected as trump for the game.
    pub trump: Card,
    /// Players that play the game.
    pub opponents: Vec<Opponent>,
}

/// Event that is sent when the game is ended.
#[derive(Debug, Event)]
pub struct GameEnded {
    pub winner_id: PlayerId,
    pub winner_name: String,
}
