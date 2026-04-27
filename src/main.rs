use bevy::{
    color::palettes::tailwind::{AMBER_800, LIME_400},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod player_controller;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup, spawn_player))
        .add_observer(player_controller::on_player_root_spawned)
        .run();
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
