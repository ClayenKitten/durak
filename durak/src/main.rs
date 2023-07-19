mod card;
mod collider;
mod main_menu;
mod network;
mod round_setup;
mod ui_utils;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_egui::EguiPlugin;
use card::CardInteractionPlugin;
use durak_lib::game::hand::Hand;
use main_menu::MainMenuPlugin;
use network::NetworkPlugin;
use round_setup::RoundSetupPlugin;

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
        .add_plugins(EguiPlugin)
        .add_state::<GameScreen>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(RoundSetupPlugin)
        .add_plugins(CardInteractionPlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, startup)
        .run();
}

#[derive(Debug, Component)]
pub struct Player {
    pub _name: String,
    pub is_controlled: bool,
}

/// Marker component for card that is discarded.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Discarded;

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
    commands.spawn((
        Player {
            _name: String::from("Me"),
            is_controlled: true,
        },
        Hand::default(),
    ));
    commands.spawn((
        Player {
            _name: String::from("Not me"),
            is_controlled: false,
        },
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
