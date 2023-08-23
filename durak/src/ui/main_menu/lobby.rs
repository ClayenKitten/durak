use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::egui::{
    Align, Button, Color32, Direction, Frame, Label, Layout, Margin, Sense, Ui, Vec2,
};
use durak_lib::{
    identifiers::PlayerId,
    status::{lobby::LobbyStatus, StatusResponse},
};

use crate::{
    network::{LeaveGameRequest, OnResponse, StartGameRequest, StatusRequest},
    session::Session,
    ui::{
        utils::{BUTTON_SIZE, MARGIN},
        UiContext,
    },
    GameScreen, GameStarted,
};

use super::CurrentScreen;

pub struct LobbyScreen;

impl Plugin for LobbyScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    display.run_if(resource_exists::<LobbyStatus>()),
                    request_status.run_if(on_timer(Duration::from_secs_f32(0.5))),
                )
                    .run_if(resource_exists::<Session>()),
                display_loading.run_if(
                    not(resource_exists::<Session>())
                        .or_else(not(resource_exists::<LobbyStatus>())),
                ),
                on_status_response,
            )
                .run_if(in_state(CurrentScreen::Lobby)),
        );
    }
}

fn display(
    mut ctx: UiContext,
    mut commands: Commands,
    status: Res<LobbyStatus>,
    session: Res<Session>,
    mut menu_state: ResMut<NextState<CurrentScreen>>,
) {
    ctx.show(|ui: &mut Ui| {
        ui.vertical_centered_justified(|ui| {
            Frame::none().fill(Color32::from_gray(15)).show(ui, |ui| {
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 75.), Sense::hover());
                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                        ui.label(format!("Game #{}", session.game));
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
                        player_entry(ui, player.id, &player.name, session.is_host);
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
                                commands.spawn(LeaveGameRequest(session.into_header()));
                                menu_state.0 = Some(CurrentScreen::MainMenu);
                            }
                            ui.add_space(ui.available_width() - BUTTON_SIZE.x);
                            if ui
                                .add_enabled(
                                    status.can_start() && session.is_host,
                                    Button::new("Start").min_size(BUTTON_SIZE),
                                )
                                .clicked()
                            {
                                commands.spawn(StartGameRequest(session.into_header()));
                            }
                        },
                    );
                });
        });
    })
}

fn request_status(mut commands: Commands, session: Res<Session>) {
    commands.spawn(StatusRequest(session.into_header()));
}

fn display_loading(mut ctx: UiContext) {
    ctx.show(|ui| {
        ui.centered_and_justified(|ui| ui.label("Waiting for server..."));
    })
}

fn player_entry(ui: &mut Ui, player: PlayerId, name: &str, is_host: bool) {
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
                    ui.label(name);
                    if is_host {
                        ui.add_space(ui.available_width() - HEIGHT);
                        ui.add(Button::new("Kick").min_size(Vec2::splat(HEIGHT)));
                    }
                });
            });
        });
}

fn on_status_response(
    mut commands: Commands,
    mut responses: EventReader<OnResponse<StatusRequest>>,
    mut state: ResMut<NextState<GameScreen>>,
    mut started: EventWriter<GameStarted>,
) {
    let Some(OnResponse(status)) = responses.iter().last() else {
        return;
    };

    match status {
        StatusResponse::Lobby(lobby) => {
            commands.insert_resource(lobby.clone());
        }
        StatusResponse::Round(round) => {
            started.send(GameStarted {
                trump: round.trump,
                opponents: round.opponents.clone(),
            });
            state.0 = Some(GameScreen::Round);
            commands.remove_resource::<LobbyStatus>();
        }
        StatusResponse::Finished(_) | StatusResponse::Error(_) => {
            state.0 = Some(GameScreen::MainMenu);
            commands.remove_resource::<LobbyStatus>();
        }
    }
}
