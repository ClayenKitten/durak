pub mod game;
pub mod main_menu;
pub mod utils;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::{
    egui::{CentralPanel, Frame, Margin, Ui},
    EguiContexts, EguiPlugin,
};

use self::main_menu::MainMenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, MainMenuPlugin));
    }
}

#[derive(SystemParam)]
pub struct UiContext<'w, 's> {
    margin: Local<'s, Margin>,
    contexts: EguiContexts<'w, 's>,
}

impl UiContext<'_, '_> {
    pub fn margin(&mut self, margin: impl Into<Margin>) -> &mut Self {
        *self.margin = margin.into();
        self
    }

    pub fn show(&mut self, show: impl FnOnce(&mut Ui)) {
        let ctx = self.contexts.ctx_mut();
        CentralPanel::default()
            .frame(
                Frame::none()
                    .inner_margin(*self.margin)
                    .fill(ctx.style().visuals.panel_fill),
            )
            .show(ctx, |ui| {
                for (_, font_id) in ui.style_mut().text_styles.iter_mut() {
                    font_id.size = 30.;
                }
                show(ui);
            });
    }
}
