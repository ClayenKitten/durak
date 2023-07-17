use bevy_egui::egui;
use bevy_egui::egui::{Align, TextEdit, Vec2, Widget};

pub struct BigTextInput<'t> {
    text: &'t mut String,
}

impl<'t> BigTextInput<'t> {
    pub fn new(text: &'t mut String) -> Self {
        Self { text }
    }
}

impl Widget for BigTextInput<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            TextEdit::singleline(self.text)
                .min_size(Vec2::new(400., 50.))
                .vertical_align(Align::Center)
                .horizontal_align(Align::Center),
        )
    }
}
