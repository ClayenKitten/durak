use bevy::prelude::*;
use bevy_egui::egui::{Button, Ui, Vec2};
use durak_lib::network::{AuthHeader, CreateGameData, CreateGameResponse};

use crate::{
    network::{CreateGameRequest, OnResponse},
    ui::{
        utils::{BigTextInput, BUTTON_SIZE, MARGIN},
        UiContext,
    },
};

use super::{CurrentScreen, IsHost};

pub struct CreateGameScreen;

impl Plugin for CreateGameScreen {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ScreenState>().add_systems(
            Update,
            (
                display.run_if(in_state(CurrentScreen::CreateGame)),
                on_create_response,
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
    ctx.margin(MARGIN).show(|ui: &mut Ui| {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0., 10.);

            ui.label("Password:");
            ui.add(BigTextInput::new(&mut state.password));
            ui.add_space(ui.available_height() - BUTTON_SIZE.y);
            ui.horizontal(|ui| {
                if ui
                    .add(Button::new("Return").min_size(BUTTON_SIZE))
                    .clicked()
                {
                    next_state.0 = Some(CurrentScreen::MainMenu);
                }
                ui.add_space(ui.available_width() - BUTTON_SIZE.x);
                if ui
                    .add(Button::new("Create").min_size(BUTTON_SIZE))
                    .clicked()
                {
                    commands.spawn(CreateGameRequest(CreateGameData {
                        password: state.password.clone(),
                    }));
                    next_state.0 = Some(CurrentScreen::Lobby);
                }
            });
        });
    });
}

fn on_create_response(
    mut commands: Commands,
    mut events: EventReader<OnResponse<CreateGameRequest>>,
    mut next_menu_state: ResMut<NextState<CurrentScreen>>,
) {
    if let Some(OnResponse(response)) = events.iter().next() {
        match response {
            CreateGameResponse::Ok {
                game_id,
                player_id,
                token,
            } => {
                commands.insert_resource(AuthHeader {
                    game_id: *game_id,
                    player_id: *player_id,
                    token: *token,
                });
                commands.insert_resource(IsHost(true));
                next_menu_state.0 = Some(CurrentScreen::Lobby);
            }
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
struct ScreenState {
    pub password: String,
}
