//! Collider used for mouse interactions.
//! 
//! Should be replaced by [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking)
//! when one is updated to bevy 0.11.0.

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Collider(pub Rect);

impl From<Rect> for Collider {
    fn from(val: Rect) -> Self {
        Collider(val)
    }
}

#[derive(Event, Clone, Copy, PartialEq, Eq)]
pub struct ClickedEvent(pub Entity);

pub fn cursor_system(
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    input: Res<Input<MouseButton>>,
    colliders: Query<(Entity, &Collider)>,
    mut event_writer: EventWriter<ClickedEvent>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (entity, collider) in colliders.iter() {
            if !collider.0.contains(world_position) {
                continue;
            }
            if !input.just_pressed(MouseButton::Left) {
                continue;
            }
            event_writer.send(ClickedEvent(entity));
        }
    }
}
