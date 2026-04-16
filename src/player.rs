use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

const PLAYER_RADIUS: f32 = 12.5;
const PLAYER_LENGTH: f32 = 20.0;

#[derive(Component, Default, Reflect)]
#[require(Transform, Visibility)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
#[require(
    RigidBody::Kinematic,
    CustomPositionIntegration,
    LockedAxes::ROTATION_LOCKED,
    SpeculativeMargin(0.0)
)]
struct CharacterController;

#[derive(Component)]
struct CharacterMovementSettings {
    max_run_speed: Scalar,
    run_acceleration: Scalar,
    run_damping: Scalar,
    jump_speed: Scalar,
    gravity: Vector,
    terminal_velocity: Scalar,
}

impl Default for CharacterMovementSettings {
    fn default() -> Self {
        Self {
            max_run_speed: 225.0,
            run_acceleration: 1800.0,
            run_damping: 12.0,
            jump_speed: 450.0,
            gravity: Vector::new(0.0, -1400.0),
            terminal_velocity: 900.0,
        }
    }
}

#[derive(Component)]
struct GroundDetection {
    max_angle: Scalar,
    max_distance: Scalar,
    cast_shape: Collider,
}

impl Default for GroundDetection {
    fn default() -> Self {
        let mut cast_shape = Collider::capsule(PLAYER_RADIUS, PLAYER_LENGTH);
        cast_shape.set_scale(Vector::splat(0.98), 10);

        Self {
            max_angle: PI / 6.0,
            max_distance: 4.0,
            cast_shape,
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Grounded;

#[derive(Resource, Default)]
struct PlayerInputState {
    horizontal: Scalar,
    jump_queued: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .init_resource::<PlayerInputState>()
        .add_systems(Startup, spawn_player)
        .add_systems(PreUpdate, collect_player_input)
        .add_systems(
            FixedUpdate,
            (
                update_grounded,
                apply_jump,
                apply_horizontal_movement,
                apply_gravity,
                apply_horizontal_damping,
                move_player,
                clear_jump_queue,
            )
                .chain(),
        );
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        CharacterController,
        CharacterMovementSettings::default(),
        GroundDetection::default(),
        LinearVelocity::ZERO,
        Collider::capsule(PLAYER_RADIUS, PLAYER_LENGTH),
        Mesh2d(meshes.add(Capsule2d::new(PLAYER_RADIUS, PLAYER_LENGTH))),
        MeshMaterial2d(materials.add(Color::srgb(0.82, 0.24, 0.22))),
        Transform::from_xyz(0.0, -110.0, 1.0),
    ));
}

fn collect_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input_state: ResMut<PlayerInputState>,
) {
    let left = keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]);
    let right = keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]);

    input_state.horizontal = (right as i8 - left as i8) as Scalar;
    input_state.jump_queued |= keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::KeyW]);
}

fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &GroundDetection, &GlobalTransform), With<CharacterController>>,
    spatial_query: SpatialQuery,
) {
    for (entity, ground_detection, transform) in &mut query {
        let rotation = Rotation::from(transform.rotation());
        let hit = spatial_query.cast_shape(
            &ground_detection.cast_shape,
            transform.translation().xy().adjust_precision(),
            rotation.as_radians(),
            Dir2::NEG_Y,
            &ShapeCastConfig::from_max_distance(ground_detection.max_distance),
            &SpatialQueryFilter::from_excluded_entities([entity]),
        );

        let is_grounded = hit.is_some_and(|hit| {
            let up = transform.up().xy().adjust_precision();
            hit.normal1.angle_to(up).abs() <= ground_detection.max_angle
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn apply_jump(
    input_state: Res<PlayerInputState>,
    mut query: Query<(&CharacterMovementSettings, &mut LinearVelocity), With<Grounded>>,
) {
    if !input_state.jump_queued {
        return;
    }

    for (movement, mut linear_velocity) in &mut query {
        linear_velocity.y = movement.jump_speed;
    }
}

fn apply_horizontal_movement(
    time: Res<Time>,
    input_state: Res<PlayerInputState>,
    mut query: Query<(&CharacterMovementSettings, &mut LinearVelocity), With<CharacterController>>,
) {
    let delta_secs = time.delta_secs_f64().adjust_precision();

    if input_state.horizontal == 0.0 {
        return;
    }

    for (movement, mut linear_velocity) in &mut query {
        linear_velocity.x += input_state.horizontal * movement.run_acceleration * delta_secs;
        linear_velocity.x = linear_velocity
            .x
            .clamp(-movement.max_run_speed, movement.max_run_speed);
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&CharacterMovementSettings, &mut LinearVelocity), With<CharacterController>>,
) {
    let delta_secs = time.delta_secs_f64().adjust_precision();

    for (movement, mut linear_velocity) in &mut query {
        linear_velocity.0 += movement.gravity * delta_secs;
        linear_velocity.y = linear_velocity
            .y
            .max(-movement.terminal_velocity)
            .min(movement.jump_speed);
    }
}

fn apply_horizontal_damping(
    time: Res<Time>,
    input_state: Res<PlayerInputState>,
    mut query: Query<(&CharacterMovementSettings, &mut LinearVelocity), With<CharacterController>>,
) {
    if input_state.horizontal != 0.0 {
        return;
    }

    let delta_secs = time.delta_secs_f64().adjust_precision();

    for (movement, mut linear_velocity) in &mut query {
        linear_velocity.x *= 1.0 / (1.0 + movement.run_damping * delta_secs);

        if linear_velocity.x.abs() < 0.5 {
            linear_velocity.x = 0.0;
        }
    }
}

fn move_player(
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut query: Query<
        (Entity, &Collider, &mut Transform, &mut LinearVelocity),
        With<CharacterController>,
    >,
) {
    for (entity, collider, mut transform, mut linear_velocity) in &mut query {
        let MoveAndSlideOutput {
            position,
            projected_velocity,
        } = move_and_slide.move_and_slide(
            collider,
            transform.translation.xy().adjust_precision(),
            Rotation::from(transform.rotation).as_radians(),
            linear_velocity.0,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &SpatialQueryFilter::from_excluded_entities([entity]),
            |_| MoveAndSlideHitResponse::Accept,
        );

        transform.translation = position.f32().extend(transform.translation.z);
        linear_velocity.0 = projected_velocity;
    }
}

fn clear_jump_queue(mut input_state: ResMut<PlayerInputState>) {
    input_state.jump_queued = false;
}
