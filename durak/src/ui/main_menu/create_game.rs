use bevy::prelude::*;
use bevy_egui::egui::{Button, Ui, Vec2};
use durak_lib::network::{CreateGameData, CreateGameResponse};

use crate::{
    network::{CreateGameRequest, OnResponse},
    persistence::Configuration,
    session::Session,
    ui::{
        utils::{BigTextInput, BUTTON_SIZE, MARGIN},
        UiContext,
    },
};

use super::CurrentScreen;

pub struct CreateGameScreen;

impl Plugin for CreateGameScreen {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ScreenState>()
            .add_systems(
                Update,
                (
                    display.run_if(in_state(CurrentScreen::CreateGame)),
                    on_create_response,
                ),
            )
            .add_systems(
                OnEnter(CurrentScreen::CreateGame),
                |mut screen: ResMut<ScreenState>| {
                    screen.password = String::new();
                },
            );
    }
}

fn display(
    mut ctx: UiContext,
    mut commands: Commands,
    mut state: ResMut<ScreenState>,
    mut config: ResMut<Configuration>,
    mut next_state: ResMut<NextState<CurrentScreen>>,
) {
    ctx.margin(MARGIN).show(|ui: &mut Ui| {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0., 10.);

            ui.label("Name:");
            if ui.add(BigTextInput::new(&mut config.name)).lost_focus() {
                let _ = config.save();
            }
            ui.add_space(25.);

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
                        name: config.name.clone(),
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
    config: Res<Configuration>,
    mut next_menu_state: ResMut<NextState<CurrentScreen>>,
) {
    if let Some(OnResponse(response)) = events.iter().next() {
        match response {
            CreateGameResponse::Ok {
                game_id,
                player_id,
                token,
            } => {
                commands.insert_resource(Session {
                    name: config.name.clone(),
                    id: *player_id,
                    game: *game_id,
                    token: *token,
                    is_host: true,
                });
                next_menu_state.0 = Some(CurrentScreen::Lobby);
            }
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
struct ScreenState {
    pub password: String,
}
