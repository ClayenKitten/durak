use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Button, CentralPanel, RichText, Ui, Vec2},
    EguiContexts,
};
use durak_lib::{
    common::{GameId, PlayerId},
    network::{CreateGameData, JoinGameData},
};

use crate::{
    network::{CreateGameRequest, JoinGameRequest},
    ui_utils::BigTextInput,
    GameScreen,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>().add_systems(
            Update,
            display_main_menu.run_if(in_state(GameScreen::MainMenu)),
        );
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum MenuState {
    #[default]
    MainMenu,
    CreateGame {
        password: String,
    },
    JoinGame {
        id: String,
        password: String,
    },
    Lobby {
        id: Option<GameId>,
    },
    None,
}

fn display_main_menu(
    mut ctx: EguiContexts,
    commands: Commands,
    next_game_state: ResMut<NextState<GameScreen>>,
    exit: EventWriter<AppExit>,
    mut menu_state: ResMut<MenuState>,
) {
    if menu_state.as_ref() == &MenuState::None {
        return;
    }
    let mut next_state: Option<MenuState> = None;
    let ctx = ctx.ctx_mut();
    CentralPanel::default().show(ctx, |ui| {
        for (_, font_id) in ui.style_mut().text_styles.iter_mut() {
            font_id.size = 30.;
        }
        ui.horizontal_centered(|ui| {
            ui.add_space(50.);
            ui.allocate_ui(
                Vec2 {
                    x: ui.available_width() - 50.,
                    y: ui.available_height(),
                },
                |ui| {
                    ui.vertical(|ui| match menu_state.as_mut() {
                        MenuState::MainMenu => {
                            main_menu(ui, &mut next_state, exit);
                        }
                        MenuState::CreateGame { password } => {
                            create_game(ui, commands, password, &mut next_state);
                        }
                        MenuState::JoinGame { id, password } => {
                            join_game(ui, commands, id, password, &mut next_state);
                        }
                        MenuState::Lobby { id } => {
                            lobby(
                                ui,
                                &mut next_state,
                                next_game_state,
                                *id,
                                vec![PlayerId::new(0), PlayerId::new(1)],
                            );
                        }
                        MenuState::None => unreachable!(),
                    });
                },
            );
        });
    });
    if let Some(next_state) = next_state {
        *menu_state = next_state;
    }
}

const BUTTON_SIZE: Vec2 = Vec2::new(400., 100.);

fn main_menu(ui: &mut Ui, next_state: &mut Option<MenuState>, mut exit: EventWriter<AppExit>) {
    let height = ui.available_size().y / 2. - (100. * 3. + 50. * 2.) / 2.;
    ui.add_space(height);

    ui.vertical_centered(|ui| {
        ui.style_mut().spacing.item_spacing = Vec2::new(0., 50.);

        if ui
            .add(Button::new("Create").min_size(BUTTON_SIZE))
            .clicked()
        {
            *next_state = Some(MenuState::CreateGame {
                password: String::new(),
            });
        }
        if ui.add(Button::new("Join").min_size(BUTTON_SIZE)).clicked() {
            *next_state = Some(MenuState::JoinGame {
                id: String::new(),
                password: String::new(),
            });
        }
        if ui.add(Button::new("Quit").min_size(BUTTON_SIZE)).clicked() {
            exit.send(AppExit);
        }
    });
}

fn create_game(
    ui: &mut Ui,
    mut commands: Commands,
    password: &mut String,
    next_state: &mut Option<MenuState>,
) {
    ui.add_space(50.);

    ui.vertical_centered(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(0., 10.);

        ui.label(RichText::new("Password:").size(30.));
        ui.add(BigTextInput::new(password));
        ui.add_space(ui.available_height() - 150.);
        ui.horizontal(|ui| {
            if ui
                .add(Button::new("Return").min_size(BUTTON_SIZE))
                .clicked()
            {
                *next_state = Some(MenuState::MainMenu);
            }
            ui.add_space(ui.available_width() - BUTTON_SIZE.x);
            if ui
                .add(Button::new("Create").min_size(BUTTON_SIZE))
                .clicked()
            {
                commands.spawn(CreateGameRequest(CreateGameData {
                    password: password.clone(),
                }));
                *next_state = Some(MenuState::Lobby { id: None });
            }
        });
    });
}

fn join_game(
    ui: &mut Ui,
    mut commands: Commands,
    id: &mut String,
    password: &mut String,
    next_state: &mut Option<MenuState>,
) {
    ui.style_mut().spacing.item_spacing = Vec2::new(50., 50.);

    ui.add_space(50.);
    ui.columns(2, |columns| {
        for column in columns.iter_mut() {
            column.spacing_mut().item_spacing = Vec2::new(0., 10.);
        }
        columns[0].label("Id:");
        columns[0].add(BigTextInput::new(id));
        columns[1].label("Password:");
        columns[1].add(BigTextInput::new(password));
    });
    ui.add_space(ui.available_height() - 150.);
    ui.horizontal(|ui| {
        if ui
            .add(Button::new("Return").min_size(BUTTON_SIZE))
            .clicked()
        {
            *next_state = Some(MenuState::MainMenu);
        }

        ui.add_space(ui.available_width() - BUTTON_SIZE.x);

        if ui.add(Button::new("Join").min_size(BUTTON_SIZE)).clicked() {
            let id = match id.parse::<u64>() {
                Ok(id) => GameId::new(id),
                Err(_) => return,
            };
            commands.spawn(JoinGameRequest(JoinGameData {
                id,
                password: password.clone(),
            }));
            *next_state = Some(MenuState::Lobby { id: None });
        }
    });
}

fn lobby(
    ui: &mut Ui,
    next_state: &mut Option<MenuState>,
    next_game_state: ResMut<NextState<GameScreen>>,
    game_id: Option<GameId>,
    players: Vec<PlayerId>,
) {
    match game_id {
        Some(id) => todo!(),
        None => {
            ui.centered_and_justified(|ui| ui.label("Waiting for server..."));
        }
    };
}
