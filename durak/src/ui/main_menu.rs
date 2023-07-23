mod create_game;
mod join_game;
mod lobby;
mod main;

use self::{
    create_game::CreateGameScreen, join_game::JoinGameScreen, lobby::LobbyScreen, main::MainScreen,
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CurrentScreen>()
            .add_plugins(MainScreen)
            .add_plugins(CreateGameScreen)
            .add_plugins(JoinGameScreen)
            .add_plugins(LobbyScreen);
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct IsHost(pub bool);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum CurrentScreen {
    #[default]
    MainMenu,
    CreateGame,
    JoinGame,
    Lobby,
    None,
}
