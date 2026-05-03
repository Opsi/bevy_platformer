use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerBottomBox;

#[derive(Component)]
pub struct PlayerBottomBoxCast;

#[derive(Component)]
pub struct PlayerBodyCollider;

#[derive(Component)]
#[require(Transform)]
pub struct PlayerRoot;
