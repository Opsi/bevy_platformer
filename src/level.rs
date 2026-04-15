use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        // Visuals
        Mesh2d(meshes.add(Rectangle::new(400.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.2))),
        Transform::from_xyz(0.0, -200.0, 0.0),
        // Physics
        RigidBody::Static,
        Collider::rectangle(400.0, 50.0),
    ));
}
