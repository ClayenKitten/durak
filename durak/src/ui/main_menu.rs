mod create_game;
mod join_game;
mod lobby;
mod main;

use crate::GameScreen;

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
            .add_plugins(LobbyScreen)
            .add_systems(OnEnter(GameScreen::MainMenu), reset_screen);
    }
}

fn reset_screen(mut next_state: ResMut<NextState<CurrentScreen>>) {
    next_state.0 = Some(CurrentScreen::MainMenu);
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum CurrentScreen {
    #[default]
    MainMenu,
    CreateGame,
    JoinGame,
    Lobby,
    None,
}
