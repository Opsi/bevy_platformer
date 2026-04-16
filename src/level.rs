use avian2d::prelude::*;
use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::PrimitiveTopology};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground_material = materials.add(Color::srgb(0.22, 0.61, 0.30));
    let ramp_material = materials.add(Color::srgb(0.45, 0.48, 0.55));
    let platform_material = materials.add(Color::srgb(0.78, 0.76, 0.68));

    spawn_rect(
        &mut commands,
        &mut meshes,
        ground_material.clone(),
        Vec2::new(36.0, 2.0),
        Vec2::new(0.0, -4.5),
    );

    spawn_rect(
        &mut commands,
        &mut meshes,
        ground_material.clone(),
        Vec2::new(3.0, 1.0),
        Vec2::new(-4.5, -2.5),
    );
    spawn_rect(
        &mut commands,
        &mut meshes,
        ground_material.clone(),
        Vec2::new(3.0, 1.4),
        Vec2::new(-1.0, -2.3),
    );
    spawn_rect(
        &mut commands,
        &mut meshes,
        ground_material.clone(),
        Vec2::new(3.2, 1.8),
        Vec2::new(2.8, -2.1),
    );

    spawn_rect(
        &mut commands,
        &mut meshes,
        platform_material.clone(),
        Vec2::new(4.5, 0.5),
        Vec2::new(-7.0, 0.0),
    );
    spawn_rect(
        &mut commands,
        &mut meshes,
        platform_material.clone(),
        Vec2::new(3.5, 0.45),
        Vec2::new(5.5, 1.0),
    );
    spawn_rect(
        &mut commands,
        &mut meshes,
        platform_material.clone(),
        Vec2::new(3.0, 0.45),
        Vec2::new(10.5, 3.0),
    );
    spawn_rect(
        &mut commands,
        &mut meshes,
        platform_material.clone(),
        Vec2::new(2.8, 0.45),
        Vec2::new(-1.5, 3.6),
    );

    spawn_rect(
        &mut commands,
        &mut meshes,
        ramp_material.clone(),
        Vec2::new(1.0, 4.5),
        Vec2::new(14.5, -1.25),
    );

    spawn_ramp(
        &mut commands,
        &mut meshes,
        ramp_material.clone(),
        Vec2::new(-11.5, -3.5),
        Vec2::new(-6.5, -3.5),
        Vec2::new(-11.5, -1.8),
    );
    spawn_ramp(
        &mut commands,
        &mut meshes,
        ramp_material,
        Vec2::new(7.0, -3.5),
        Vec2::new(12.0, -3.5),
        Vec2::new(12.0, -0.4),
    );
}

fn spawn_rect(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<ColorMaterial>,
    size: Vec2,
    position: Vec2,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(material),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    ));
}

fn spawn_ramp(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<ColorMaterial>,
    a: Vec2,
    b: Vec2,
    c: Vec2,
) {
    let centroid = (a + b + c) / 3.0;
    let local_a = a - centroid;
    let local_b = b - centroid;
    let local_c = c - centroid;

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [local_a.x, local_a.y, 0.0],
            [local_b.x, local_b.y, 0.0],
            [local_c.x, local_c.y, 0.0],
        ],
    );

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(material),
        Transform::from_xyz(centroid.x, centroid.y, 0.0),
        RigidBody::Static,
        Collider::triangle(local_a, local_b, local_c),
    ));
}
