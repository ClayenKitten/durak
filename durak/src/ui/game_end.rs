use bevy::prelude::*;

use crate::{GameEnded, GameScreen};

use super::UiContext;

pub fn show_game_end_ui(
    mut ctx: UiContext,
    mut events: EventReader<GameEnded>,
    mut winner: Local<Option<String>>,
    mut next_state: ResMut<NextState<GameScreen>>,
) {
    if let Some(GameEnded { winner_name, .. }) = events.iter().last() {
        *winner = Some(winner_name.clone());
    };

    if let Some(winner_name) = winner.clone() {
        ctx.show(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.);
                ui.spacing_mut().item_spacing.y = 50.;

                ui.label("Game over!");
                ui.label(format!("{winner_name} won"));
                if ui.button("Leave to menu").clicked() {
                    *winner = None;
                    next_state.0 = Some(GameScreen::MainMenu);
                }
            });
        });
    }
}
