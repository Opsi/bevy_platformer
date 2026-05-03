use avian2d::{math::*, prelude::*};
use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, ORANGE_RED},
        tailwind::{CYAN_500, ORANGE_500},
    },
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_platformer::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Update, mouse_follow_system)
        .add_systems(Update, render_rays)
        .add_systems(Startup, setup)
        .add_observer(on_player_root_spawned)
        .run();
}

#[derive(Component)]
struct MouseFollower;

fn mouse_follow_system(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_follower: Query<&mut Transform, With<MouseFollower>>,
) {
    let window = q_window.single().unwrap();
    let (camera, camera_transform) = q_camera.single().unwrap();
    let mut follower_transform = q_follower.single_mut().unwrap();

    // 1. Get cursor position from the window
    if let Some(cursor_pos) = window.cursor_position() {
        // 2. Convert viewport (screen) coordinates to world coordinates
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // 3. Update the follower's translation
            follower_transform.translation.x = world_pos.x;
            follower_transform.translation.y = world_pos.y;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // spawn a rectangle
    commands.spawn((
        Name::new("Solid Rectangle"),
        Mesh2d(meshes.add(Rectangle::new(50., 20.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Transform::from_xyz(500., 200., 0.0),
        Collider::rectangle(50., 20.),
    ));

    // spawn a circle
    commands.spawn((
        Name::new("Solid Circle"),
        Mesh2d(meshes.add(Circle::new(30.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Transform::from_xyz(-500., -200., 0.0),
        Collider::circle(30.),
    ));

    // spawn the mouse-controlled, raycasted rectangle
    let body_width = 80.;
    let body_height = 150.;
    let feet_casts = 5;
    let body_rect = Rectangle::new(body_width, body_height);
    commands.spawn((
        Name::new("Ma-Boi"),
        MouseFollower,
        PlayerRoot,
        RigidBody::Kinematic,
        Mesh2d(meshes.add(body_rect)),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Transform::default(),
        Collider::rectangle(body_width, body_height),
    ));
    // .with_children(|parent| {
    //     let step_width = body_width / (feet_casts - 1) as f32;
    //     let query_filter = SpatialQueryFilter::default().with_excluded_entities([parent]);
    //
    //     for i in 0..feet_casts {
    //         let x = (i as f32 * step_width) - body_rect.half_size.x;
    //         parent.spawn((
    //             Name::new(format!("PlayerBottomBoxRaycaster-{}", i)),
    //             RayCaster::new(Vec2::new(x, -body_rect.half_size.y), Dir2::SOUTH)
    //                 .with_max_distance(40.),
    //         ));
    //     }
    // });
}

pub fn on_player_root_spawned(add: On<Add, PlayerRoot>, mut commands: Commands) {
    let body_width = 80.;
    let body_height = 150.;
    let feet_casts = 5;
    let body_rect = Rectangle::new(body_width, body_height);
    commands.entity(add.entity).with_children(|parent| {
        let step_width = body_width / (feet_casts - 1) as f32;
        let query_filter = SpatialQueryFilter::default().with_excluded_entities([add.entity]);

        for i in 0..feet_casts {
            let x = (i as f32 * step_width) - body_rect.half_size.x;
            parent.spawn((
                Name::new(format!("PlayerBottomBoxRaycaster-{}", i)),
                RayCaster::new(Vec2::new(x, -body_rect.half_size.y), Dir2::SOUTH)
                    .with_max_distance(40.)
                    .with_query_filter(query_filter.clone()),
            ));
        }
    });
}

// Note: The `PhysicsDebugPlugin` can also render rays, hit points, and normals.
//       This system is primarily for demonstration purposes.
fn render_rays(mut rays: Query<(&mut RayCaster, &mut RayHits)>, mut gizmos: Gizmos) {
    let mut count = 0;
    for (ray, hits) in &mut rays {
        count += 1;
        // Convert to Vec3 for lines
        let origin = ray.global_origin().f32();
        let direction = ray.global_direction().f32();

        gizmos.line_2d(origin, origin + direction * ray.max_distance, ORANGE_RED);
        for hit in hits.iter() {
            gizmos.line_2d(origin, origin + direction * hit.distance as f32, GREEN);
        }
    }
}
