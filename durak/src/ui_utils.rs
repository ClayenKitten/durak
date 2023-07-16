use bevy::prelude::Color;

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::rgb(0.2, 0.2, 0.2);
    pub const SECONDARY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const BACKGROUND: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const TEXT: Color = Color::rgb(0.9, 0.9, 0.9);
}
