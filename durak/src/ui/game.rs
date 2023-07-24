//! Ui that is used during the game.

use bevy::prelude::*;
use bevy_egui::{
    egui::{Button, CentralPanel, Frame, Vec2},
    EguiContexts,
};

use crate::{
    network::{RetreatRequest, TakeRequest},
    session::Session,
    ui::utils::MARGIN,
};

const BUTTON_SIZE: Vec2 = Vec2::new(50., 50.);

pub fn display_ui(mut ctx: EguiContexts, mut commands: Commands, session: Res<Session>) {
    let ctx = ctx.ctx_mut();
    CentralPanel::default()
        .frame(Frame::none().inner_margin(MARGIN))
        .show(ctx, |ui| {
            ui.add_space(ui.available_height() / 2. - BUTTON_SIZE.y / 2.);
            ui.horizontal(|ui| {
                ui.add_space(100.);
                if ui.add(Button::new("Take").min_size(BUTTON_SIZE)).clicked() {
                    commands.spawn(TakeRequest(session.into_header()));
                }
                if ui
                    .add(Button::new("Retreat").min_size(BUTTON_SIZE))
                    .clicked()
                {
                    commands.spawn(RetreatRequest(session.into_header()));
                }
            })
        });
}
