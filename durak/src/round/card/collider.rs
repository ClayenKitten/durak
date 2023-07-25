//! Collider used for mouse interactions.
//!
//! Should be replaced by [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking)
//! when one is updated to bevy 0.11.0.

use bevy::prelude::*;
use durak_lib::game::card::Card;

use super::interaction::CardClicked;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Collider(Rect);

impl Collider {
    /// Creates new collider from offset and size.
    pub fn new(size: Vec2) -> Self {
        Self(Rect::from_center_size(Vec2::ZERO, size))
    }

    /// Returns `true` if point is inside collider with provided translation.
    pub fn contains(&self, translation: Vec3, point: Vec2) -> bool {
        let Rect { mut min, mut max } = self.0;
        min += translation.truncate();
        max += translation.truncate();
        let translated_rect = Rect { min, max };
        translated_rect.contains(point)
    }
}

pub fn cursor_system(
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    input: Res<Input<MouseButton>>,
    colliders: Query<(Entity, &GlobalTransform, &Collider)>,
    cards: Query<(), With<Card>>,
    mut event_writer: EventWriter<CardClicked>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    let mouse_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    let Some(mouse_position) = mouse_position else {
        return;
    };

    for (entity, transform, collider) in colliders.iter() {
        if !collider.contains(transform.translation(), mouse_position) {
            continue;
        }
        if !input.just_pressed(MouseButton::Left) {
            continue;
        }
        if !cards.contains(entity) {
            continue;
        }
        event_writer.send(CardClicked(entity));
    }
}
