use std::time::Duration;

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{
        Align, Button, CentralPanel, Color32, Direction, Frame, Label, Layout, Margin, RichText,
        Sense, Ui, Vec2,
    },
    EguiContexts,
};
use durak_lib::{
    common::{GameId, PlayerId},
    network::{AuthHeader, CreateGameData, CreateGameResponce, JoinGameData, JoinGameResponce, GameState},
};

use crate::{
    network::{CreateGameRequest, JoinGameRequest, LeaveGameRequest, OnResponce, StatusRequest},
    ui_utils::BigTextInput,
    GameScreen,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>()
            .init_resource::<LobbyStatus>()
            .init_resource::<StatusRequestTimer>()
            .add_systems(
                Update,
                (
                    request_status.run_if(resource_equals(MenuState::Lobby)),
                    (on_create_response, on_join_response, on_status_response),
                    (
                        display_main_menu
                            .run_if(not(resource_equals(MenuState::None)))
                            .run_if(not(resource_equals(MenuState::Lobby))),
                        display_lobby.run_if(resource_equals(MenuState::Lobby)),
                    ),
                )
                    .chain()
                    .run_if(in_state(GameScreen::MainMenu)),
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
    Lobby,
    None,
}

fn display_main_menu(
    mut ctx: EguiContexts,
    commands: Commands,
    exit: EventWriter<AppExit>,
    mut menu_state: ResMut<MenuState>,
) {
    let mut next_state: Option<MenuState> = None;
    let ctx = ctx.ctx_mut();
    CentralPanel::default()
        .frame(
            Frame::none()
                .inner_margin(0.)
                .fill(ctx.style().visuals.panel_fill),
        )
        .show(ctx, |ui| {
            for (_, font_id) in ui.style_mut().text_styles.iter_mut() {
                font_id.size = 30.;
            }
            ui.horizontal_centered(|ui| {
                ui.add_space(MARGIN);
                ui.allocate_ui(
                    Vec2 {
                        x: ui.available_width() - MARGIN,
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
                            MenuState::Lobby => unreachable!(),
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
const MARGIN: f32 = 50.;

fn main_menu(ui: &mut Ui, next_state: &mut Option<MenuState>, mut exit: EventWriter<AppExit>) {
    let height = ui.available_size().y / 2. - (BUTTON_SIZE.y * 3. + MARGIN * 2.) / 2.;
    ui.add_space(height);

    ui.vertical_centered(|ui| {
        ui.style_mut().spacing.item_spacing = Vec2::new(0., MARGIN);

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
    ui.add_space(MARGIN);

    ui.vertical_centered(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(0., 10.);

        ui.label(RichText::new("Password:").size(30.));
        ui.add(BigTextInput::new(password));
        ui.add_space(ui.available_height() - BUTTON_SIZE.y - MARGIN);
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
                *next_state = Some(MenuState::Lobby);
            }
        });
    });
}

fn on_create_response(
    mut commands: Commands,
    mut events: EventReader<OnResponce<CreateGameRequest>>,
    mut menu_state: ResMut<MenuState>,
) {
    match menu_state.as_ref() {
        MenuState::Lobby => {}
        _ => return,
    }

    for OnResponce(response) in events.iter() {
        match response {
            CreateGameResponce::Ok {
                game_id,
                player_id,
                token,
            } => {
                commands.insert_resource(AuthHeader {
                    game_id: *game_id,
                    player_id: *player_id,
                    token: *token,
                });
                *menu_state = MenuState::Lobby;
            }
        }
        break;
    }
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
    ui.add_space(ui.available_height() - BUTTON_SIZE.y - MARGIN);
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
            *next_state = Some(MenuState::Lobby);
        }
    });
}

fn on_join_response(
    mut commands: Commands,
    mut events: EventReader<OnResponce<JoinGameRequest>>,
    mut menu_state: ResMut<MenuState>,
) {
    match menu_state.as_ref() {
        MenuState::Lobby => {}
        _ => return,
    }

    for OnResponce(response) in events.iter() {
        match response {
            JoinGameResponce::Ok {
                game_id,
                player_id,
                token,
            } => {
                commands.insert_resource(AuthHeader {
                    game_id: *game_id,
                    player_id: *player_id,
                    token: *token,
                });
                *menu_state = MenuState::Lobby;
            }
            _ => {
                *menu_state = MenuState::MainMenu;
            }
        }
        break;
    }
}

fn display_lobby(
    mut ctx: EguiContexts,
    mut commands: Commands,
    mut menu_state: ResMut<MenuState>,
    mut next_game_state: ResMut<NextState<GameScreen>>,
    status: Res<LobbyStatus>,
    auth: Option<Res<AuthHeader>>,
) {
    let ctx = ctx.ctx_mut();

    match auth {
        None => {
            CentralPanel::default().show(ctx, |ui| {
                for (_, font_id) in ui.style_mut().text_styles.iter_mut() {
                    font_id.size = 30.;
                }
                ui.centered_and_justified(|ui| ui.label("Waiting for server..."));
            });
        }
        Some(auth) => {
            let AuthHeader {
                game_id,
                player_id,
                token,
            } = auth.as_ref();
            CentralPanel::default().show(ctx, |ui| {
                for (_, font_id) in ui.style_mut().text_styles.iter_mut() {
                    font_id.size = 30.;
                }
                ui.vertical_centered_justified(|ui| {
                    Frame::none().fill(Color32::from_gray(15)).show(ui, |ui| {
                        let (rect, _) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 75.),
                            Sense::hover(),
                        );
                        ui.allocate_ui_at_rect(rect, |ui| {
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::TopDown),
                                |ui| {
                                    ui.label(format!("Game #{game_id}"));
                                },
                            );
                        });
                    });
                    Frame::none().show(ui, |ui| {
                        let (rect, _) = ui.allocate_exact_size(
                            Vec2::new(
                                ui.available_width(),
                                ui.available_height() - BUTTON_SIZE.y - MARGIN * 2.,
                            ),
                            Sense::click(),
                        );
                        ui.allocate_ui_at_rect(rect, |ui| {
                            for player in status.players.iter() {
                                player_entry(ui, *player, true);
                            }
                        });
                    });
                    Frame::none()
                        .fill(Color32::from_gray(15))
                        .inner_margin(MARGIN)
                        .show(ui, |ui| {
                            ui.allocate_ui_with_layout(
                                Vec2::new(ui.available_width(), BUTTON_SIZE.y),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    if ui.add(Button::new("Leave").min_size(BUTTON_SIZE)).clicked()
                                    {
                                        commands.spawn(LeaveGameRequest(AuthHeader {
                                            game_id: *game_id,
                                            player_id: *player_id,
                                            token: *token,
                                        }));
                                        *menu_state = MenuState::MainMenu;
                                    }
                                    ui.add_space(ui.available_width() - BUTTON_SIZE.x);
                                    if ui.add_enabled(
                                        status.can_start,
                                        Button::new("Start").min_size(BUTTON_SIZE)
                                    ).clicked() {
                                        next_game_state.0 = Some(GameScreen::RoundSetup);
                                        *menu_state = MenuState::None;
                                    }
                                },
                            );
                        });
                });
            });
        }
    };
}

fn player_entry(ui: &mut Ui, player: PlayerId, is_host: bool) {
    const HEIGHT: f32 = 100.;
    Frame::none()
        .outer_margin(Margin::symmetric(MARGIN, MARGIN / 2.))
        .fill(Color32::from_gray(40))
        .show(ui, |ui| {
            let (rect, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), HEIGHT), Sense::hover());
            ui.allocate_ui_at_rect(rect, |ui| {
                ui.horizontal(|ui| {
                    Frame::none().fill(Color32::from_gray(60)).show(ui, |ui| {
                        ui.add_sized(Vec2::splat(HEIGHT), Label::new(player.to_string()));
                    });
                    if is_host {
                        ui.add_space(ui.available_width() - HEIGHT);
                        ui.add(Button::new("Kick").min_size(Vec2::splat(HEIGHT)));
                    }
                });
            });
        });
}

fn request_status(
    mut commands: Commands,
    time: Res<Time>,
    auth: Option<Res<AuthHeader>>,
    mut timer: ResMut<StatusRequestTimer>,
) {
    if let Some(auth) = auth {
        if timer.0.just_finished() {
            commands.spawn(StatusRequest(auth.as_ref().clone()));
        }
    }
    timer.0.tick(time.delta());
}

fn on_status_response(
    mut events: EventReader<OnResponce<StatusRequest>>,
    mut lobby_status: ResMut<LobbyStatus>,
) {
    for OnResponce(game_state) in events.iter() {
        match game_state {
            GameState::Lobby { players, can_start } => {
                *lobby_status = LobbyStatus {
                    players: players.clone(),
                    can_start: *can_start,
                };
            },
            _ => continue,
        }
    }
}

#[derive(Debug, Resource, Default)]
struct LobbyStatus {
    players: Vec<PlayerId>,
    can_start: bool,
}

#[derive(Debug, Resource)]
pub struct StatusRequestTimer(Timer);

impl Default for StatusRequestTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(5.0, TimerMode::Repeating);
        timer.tick(Duration::from_secs_f32(4.8));
        Self(timer)
    }
}
