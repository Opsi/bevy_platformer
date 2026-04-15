// components
// - Player (used to mark the player root)
// entities
// - Player - Transform
//   - Physics - Collider (Pill)
//   - View - Sprites / Animations
//
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[require(Transform, Visibility)]
#[reflect(Component)]
pub struct Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
}
