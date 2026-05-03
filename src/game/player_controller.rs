use avian2d::prelude::*;
use bevy::{
    color::palettes::{
        css::*,
        tailwind::{AMBER_300, CYAN_500, ORANGE_500},
    },
    math::bounding::*,
    prelude::*,
};

#[derive(Component)]
pub struct WalkSettings {
    pub maxWalkSpeed: f32,
    pub groundAcceleration: f32,
    pub groundDeceleration: f32,
    pub airAcceleration: f32,
    pub airDeceleration: f32,
}

#[derive(Component)]
pub struct RunSettings {
    pub maxRunSpeed: f32,
}

#[derive(Component)]
pub struct GroundCollisionSettings {
    pub groundDetectionRayLength: f32,
    pub headDetectionRayLength: f32,
}

#[derive(Component)]
pub struct PlayerBottomBox;

#[derive(Component)]
pub struct PlayerBottomBoxCast;

#[derive(Component)]
pub struct PlayerBodyCollider;

#[derive(Component)]
#[require(Transform)]
pub struct PlayerRoot;

pub fn on_player_root_spawned(
    add: On<Add, PlayerRoot>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(add.entity).with_children(|parent| {
        // spawn the player feet
        let feet_width = 60.;
        let feet_height = 40.;
        let feet_casts = 5;
        let player_feet = Rectangle::new(feet_width, feet_height);
        parent
            .spawn((
                Name::new("PlayerBottomBox"),
                PlayerBottomBox,
                Mesh2d(meshes.add(player_feet.to_ring(5.))),
                MeshMaterial2d(materials.add(Color::Srgba(ORANGE_500))),
                Transform::from_xyz(0., player_feet.half_size.y, -0.5),
                Collider::rectangle(60., 40.),
            ))
            .with_children(|feet_collider| {
                let step_width = feet_width / (feet_casts - 1) as f32;

                for i in 0..feet_casts {
                    let x = (i as f32 * step_width) - player_feet.half_size.x;
                    let raycaster =
                        RayCaster::new(Vec2::new(x, -player_feet.half_size.y), Dir2::SOUTH)
                            .with_max_distance(20.);

                    feet_collider.spawn((
                        Name::new(format!("PlayerBottomBoxRaycaster-{}", i)),
                        PlayerBottomBoxCast,
                        raycaster,
                        // Transform::from_xyz(x, -player_feet.half_size.y, 0.5),
                        // Mesh2d(meshes.add(Circle::new(2.))),
                        // MeshMaterial2d(materials.add(Color::Srgba(AMBER_300))),
                    ));
                }
            });

        // spawn the player body
        let player_body = Capsule2d::new(40., 60.);
        parent.spawn((
            Name::new("PlayerBodyCollider"),
            PlayerBodyCollider,
            Mesh2d(meshes.add(player_body.to_ring(5.))),
            MeshMaterial2d(materials.add(Color::Srgba(CYAN_500))),
            Transform::from_xyz(
                0.,
                player_body.half_length + player_body.radius + player_feet.half_size.y,
                -0.5,
            ),
            Collider::capsule(40., 60.),
        ));
    });
}

fn draw_filled_circle(gizmos: &mut Gizmos, position: Vec2, color: Srgba) {
    for r in [1., 2., 3.] {
        gizmos.circle_2d(position, r, color);
    }
}

fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(
        ray.ray.origin,
        ray.ray.origin + *ray.ray.direction * ray.max,
        WHITE,
    );
    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
}

fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time, position: Vec2) -> RayCast2d {
    let dist = 50.;
    let aabb_ray = Ray2d {
        origin: position,
        direction: Dir2::from_xy(0., -dist).unwrap(),
    };
    let ray_cast = RayCast2d::from_ray(aabb_ray, dist);
    draw_ray(gizmos, &ray_cast);
    ray_cast
}

pub fn debug_bottom_box_ray_cast(
    mut gizmos: Gizmos,
    time: Res<Time>,
    bottom_box_casts: Query<&GlobalTransform, With<PlayerBottomBoxCast>>,
) {
    for transform in bottom_box_casts.iter() {
        let ray_cast = get_and_draw_ray(
            &mut gizmos,
            &time,
            Vec2::new(transform.translation().x, transform.translation().y),
        );
        // let toi = match volume {
        //     CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
        //     CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
        // };
        // **intersects = toi.is_some();
        // if let Some(toi) = toi {
        //     draw_filled_circle(
        //         &mut gizmos,
        //         ray_cast.ray.origin + *ray_cast.ray.direction * toi,
        //         LIME,
        //     );
        // }
    }
}
