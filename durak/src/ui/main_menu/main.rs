use bevy::{app::AppExit, prelude::*};
use bevy_egui::egui::{Button, Ui, Vec2};

use crate::ui::{
    main_menu::CurrentScreen,
    utils::{BUTTON_SIZE, MARGIN},
    UiContext,
};

pub struct MainScreen;

impl Plugin for MainScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display.run_if(in_state(CurrentScreen::MainMenu)));
    }
}

fn display(
    mut ctx: UiContext,
    mut exit: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<CurrentScreen>>,
) {
    ctx.show(move |ui: &mut Ui| {
        let height = ui.available_size().y / 2. - (BUTTON_SIZE.y * 3. + MARGIN * 2.) / 2.;
        ui.add_space(height);

        ui.vertical_centered(|ui| {
            ui.style_mut().spacing.item_spacing = Vec2::new(0., MARGIN);

            if ui
                .add(Button::new("Create").min_size(BUTTON_SIZE))
                .clicked()
            {
                menu_state.0 = Some(CurrentScreen::CreateGame);
            }
            if ui.add(Button::new("Join").min_size(BUTTON_SIZE)).clicked() {
                menu_state.0 = Some(CurrentScreen::JoinGame);
            }
            if ui.add(Button::new("Quit").min_size(BUTTON_SIZE)).clicked() {
                exit.send(AppExit);
            }
        });
    });
}
