use bevy::prelude::*;

/// A resource that defines what role player currently has.
#[derive(Debug, Resource)]
pub enum Turn {
    Attacker,
    Defender,
}
