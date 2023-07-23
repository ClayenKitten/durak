use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::egui::{
    Align, Button, Color32, Direction, Frame, Label, Layout, Margin, Sense, Ui, Vec2,
};
use durak_lib::{identifiers::PlayerId, network::AuthHeader, status::GameState};

use crate::{
    network::{LeaveGameRequest, OnResponse, StartGameRequest, StateRequest},
    ui::{
        utils::{BUTTON_SIZE, MARGIN},
        UiContext,
    },
    GameScreen, GameStarted,
};

use super::{CurrentScreen, IsHost};

pub struct LobbyScreen;

impl Plugin for LobbyScreen {
    fn build(&self, app: &mut App) {
        app.init_resource::<LobbyStatus>()
            .init_resource::<StateRequestTimer>()
            .add_systems(
                Update,
                (
                    (display, request_state).run_if(resource_exists::<AuthHeader>()),
                    display_loading.run_if(not(resource_exists::<AuthHeader>())),
                    on_state_response,
                )
                    .run_if(in_state(CurrentScreen::Lobby)),
            );
    }
}

fn display(
    mut ctx: UiContext,
    mut commands: Commands,
    status: Res<LobbyStatus>,
    auth: Res<AuthHeader>,
    is_host: Option<Res<IsHost>>,
    mut menu_state: ResMut<NextState<CurrentScreen>>,
) {
    let AuthHeader {
        game_id,
        player_id,
        token,
    } = auth.as_ref();
    let IsHost(is_host) = *is_host.unwrap();

    ctx.show(|ui: &mut Ui| {
        ui.vertical_centered_justified(|ui| {
            Frame::none().fill(Color32::from_gray(15)).show(ui, |ui| {
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 75.), Sense::hover());
                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                        ui.label(format!("Game #{game_id}"));
                    });
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
                        player_entry(ui, *player, is_host);
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
                            if ui.add(Button::new("Leave").min_size(BUTTON_SIZE)).clicked() {
                                commands.spawn(LeaveGameRequest(AuthHeader {
                                    game_id: *game_id,
                                    player_id: *player_id,
                                    token: *token,
                                }));
                                menu_state.0 = Some(CurrentScreen::MainMenu);
                            }
                            ui.add_space(ui.available_width() - BUTTON_SIZE.x);
                            if ui
                                .add_enabled(
                                    status.can_start && is_host,
                                    Button::new("Start").min_size(BUTTON_SIZE),
                                )
                                .clicked()
                            {
                                commands.spawn(StartGameRequest(AuthHeader {
                                    game_id: *game_id,
                                    player_id: *player_id,
                                    token: *token,
                                }));
                            }
                        },
                    );
                });
        });
    })
}

fn display_loading(mut ctx: UiContext) {
    ctx.show(|ui| {
        ui.centered_and_justified(|ui| ui.label("Waiting for server..."));
    })
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

#[derive(Debug, Resource, Default)]
struct LobbyStatus {
    players: Vec<PlayerId>,
    can_start: bool,
}

fn request_state(
    mut commands: Commands,
    time: Res<Time>,
    auth: Res<AuthHeader>,
    mut timer: ResMut<StateRequestTimer>,
) {
    if timer.0.just_finished() {
        commands.spawn(StateRequest(auth.as_ref().clone()));
    }
    timer.0.tick(time.delta());
}

fn on_state_response(
    mut events: EventReader<OnResponse<StateRequest>>,
    mut lobby_status: ResMut<LobbyStatus>,
    mut menu_state: ResMut<NextState<CurrentScreen>>,
    mut next_game_state: ResMut<NextState<GameScreen>>,
    mut event_writer: EventWriter<GameStarted>,
) {
    for OnResponse(game_state) in events.iter() {
        match game_state {
            GameState::Lobby { players, can_start } => {
                *lobby_status = LobbyStatus {
                    players: players.clone(),
                    can_start: *can_start,
                };
            }
            GameState::Started { trump, players } => {
                menu_state.0 = Some(CurrentScreen::None);
                next_game_state.0 = Some(GameScreen::RoundSetup);
                event_writer.send(GameStarted {
                    trump: *trump,
                    players: players.clone(),
                });
            }
            _ => continue,
        }
    }
}

#[derive(Debug, Resource)]
pub struct StateRequestTimer(Timer);

impl Default for StateRequestTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(5.0, TimerMode::Repeating);
        timer.tick(Duration::from_secs_f32(4.8));
        Self(timer)
    }
}
