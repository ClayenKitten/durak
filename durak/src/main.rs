mod card;
mod collider;
mod network;
mod round;
mod session;
mod ui;

use bevy::{prelude::*, render::camera::ScalingMode};
use card::CardPlugin;
use durak_lib::{
    game::{card::Card, hand::Hand},
    identifiers::PlayerId,
};

use network::NetworkPlugin;
use round::RoundPlugin;
use ui::UiPlugin;

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
        .add_state::<GameScreen>()
        .add_plugins(UiPlugin)
        .add_plugins(RoundPlugin)
        .add_plugins(CardPlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, startup)
        .run();
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
    pub trump: Card,
    /// Players that play the game.
    pub players: Vec<PlayerId>,
}
