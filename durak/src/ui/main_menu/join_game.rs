use bevy::prelude::*;
use bevy_egui::egui::{Button, Vec2};
use durak_lib::{
    identifiers::GameId,
    network::{JoinGameData, JoinGameResponse},
};

use crate::{
    network::{JoinGameRequest, OnResponse},
    session::Session,
    ui::{
        utils::{BigTextInput, BUTTON_SIZE, MARGIN},
        UiContext,
    },
};

use super::CurrentScreen;

pub struct JoinGameScreen;

impl Plugin for JoinGameScreen {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenState>().add_systems(
            Update,
            (
                display.run_if(in_state(CurrentScreen::JoinGame)),
                on_join_response,
            ),
        );
    }
}

fn display(
    mut ctx: UiContext,
    mut commands: Commands,
    mut state: ResMut<ScreenState>,
    mut next_state: ResMut<NextState<CurrentScreen>>,
) {
    ctx.margin(MARGIN).show(|ui| {
        ui.vertical_centered_justified(|ui| {
            ui.style_mut().spacing.item_spacing = Vec2::new(0., 10.);

            ui.label("Name:");
            ui.add(BigTextInput::new(&mut state.name));
            ui.add_space(25.);

            ui.style_mut().spacing.item_spacing = Vec2::new(50., 50.);
            ui.columns(2, |columns| {
                for column in columns.iter_mut() {
                    column.spacing_mut().item_spacing = Vec2::new(0., 10.);
                }
                columns[0].label("Id:");
                columns[0].add(BigTextInput::new(&mut state.id));
                columns[1].label("Password:");
                columns[1].add(BigTextInput::new(&mut state.password));
            });
            ui.add_space(ui.available_height() - BUTTON_SIZE.y);
            ui.horizontal(|ui| {
                if ui
                    .add(Button::new("Return").min_size(BUTTON_SIZE))
                    .clicked()
                {
                    next_state.0 = Some(CurrentScreen::MainMenu);
                }

                ui.add_space(ui.available_width() - BUTTON_SIZE.x);

                let id: Result<GameId, _> = state.id.parse();
                if ui
                    .add_enabled(id.is_ok(), Button::new("Join").min_size(BUTTON_SIZE))
                    .clicked()
                {
                    let Ok(id) = id  else {
                            return;
                        };
                    commands.spawn(JoinGameRequest(JoinGameData {
                        id,
                        name: state.name.clone(),
                        password: state.password.clone(),
                    }));
                    next_state.0 = Some(CurrentScreen::Lobby);
                }
            });
        });
    });
}

fn on_join_response(
    mut commands: Commands,
    mut events: EventReader<OnResponse<JoinGameRequest>>,
    state: Res<ScreenState>,
    mut next_menu_state: ResMut<NextState<CurrentScreen>>,
) {
    if let Some(OnResponse(response)) = events.iter().next() {
        match response {
            JoinGameResponse::Ok {
                game_id,
                player_id,
                token,
            } => {
                commands.insert_resource(Session {
                    name: state.name.clone(),
                    id: *player_id,
                    game: *game_id,
                    token: *token,
                    is_host: false,
                });
                next_menu_state.0 = Some(CurrentScreen::Lobby);
            }
            _ => {
                next_menu_state.0 = Some(CurrentScreen::MainMenu);
            }
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
struct ScreenState {
    pub id: String,
    pub name: String,
    pub password: String,
}
