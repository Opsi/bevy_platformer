use avian2d::{PhysicsPlugins, prelude::*};
use bevy::{
    color::palettes::tailwind::{AMBER_800, GRAY_800},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::AppState;

mod player_controller;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), (setup, spawn_player))
        .add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default().with_length_unit(10.),
            // Enables debug rendering
            PhysicsDebugPlugin,
        ))
        .insert_gizmo_config(
            PhysicsGizmos {
                collider_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        )
        .add_systems(PostUpdate, player_controller::debug_bottom_box_ray_cast)
        .add_observer(player_controller::on_player_root_spawned);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("PlayerRoot"),
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(materials.add(Color::hsl(5., 5., 5.))),
        Transform::from_xyz(0., 0., 1.),
        player_controller::PlayerRoot,
        // these settings came from: https://www.youtube.com/watch?v=zHSWG05byEc
        RigidBody::Dynamic,
        // Enable swept CCD for this body. Considers both translational and rotational motion by default.
        // This could also be on the ball projectiles.
        SweptCcd::default(),
        Mass(0.),
        LinearDamping(0.),
        AngularDamping(0.),
        GravityScale(0.),
        TransformInterpolation,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.),
        Restitution::new(0.),
    ));

    // setup level
    commands.spawn((
        Name::new("Floor"),
        Mesh2d(meshes.add(Rectangle::new(1000., 100.))),
        MeshMaterial2d(materials.add(Color::Srgba(GRAY_800))),
        Transform::from_xyz(0., -300., -5.),
        RigidBody::Static,
        Collider::rectangle(1000., 100.),
    ));

    commands.spawn(Camera2d);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(0., 200., 0.),
        Mesh2d(meshes.add(Capsule2d::new(15., 40.))),
        MeshMaterial2d(materials.add(Color::Srgba(AMBER_800))),
    ));
}
