use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

const PLAYER_RADIUS: f32 = 0.35;
const PLAYER_LENGTH: f32 = 1.10;

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

#[derive(Component, Resource, Clone, Reflect)]
#[reflect(Component, Resource)]
pub struct CharacterMovementSettings {
    pub max_run_speed: Scalar,
    pub run_acceleration: Scalar,
    pub run_damping: Scalar,
    pub jump_speed: Scalar,
    pub gravity: Vec2,
    pub terminal_velocity: Scalar,
}

impl Default for CharacterMovementSettings {
    fn default() -> Self {
        Self {
            max_run_speed: 9.0,
            run_acceleration: 45.0,
            run_damping: 20.0,
            jump_speed: 10.5,
            gravity: Vec2::new(0.0, -24.0),
            terminal_velocity: 18.0,
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
            max_distance: 0.08,
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
        .register_type::<CharacterMovementSettings>()
        .init_resource::<PlayerInputState>()
        .init_resource::<CharacterMovementSettings>()
        .add_plugins(ResourceInspectorPlugin::<CharacterMovementSettings>::default())
        .add_systems(PreUpdate, collect_player_input)
        .add_systems(Update, sync_movement_settings)
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

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    movement_settings: &CharacterMovementSettings,
    translation: Vec3,
) -> Entity {
    commands
        .spawn((
            Player,
            CharacterController,
            movement_settings.clone(),
            GroundDetection::default(),
            LinearVelocity::ZERO,
            Collider::capsule(PLAYER_RADIUS, PLAYER_LENGTH),
            Mesh2d(meshes.add(Capsule2d::new(PLAYER_RADIUS, PLAYER_LENGTH))),
            MeshMaterial2d(materials.add(Color::srgb(0.82, 0.24, 0.22))),
            Transform::from_translation(translation),
        ))
        .id()
}

fn sync_movement_settings(
    movement_settings: Res<CharacterMovementSettings>,
    mut query: Query<&mut CharacterMovementSettings, With<CharacterController>>,
) {
    if !movement_settings.is_changed() {
        return;
    }

    for mut player_settings in &mut query {
        *player_settings = movement_settings.clone();
    }
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
        linear_velocity.0 += movement.gravity.adjust_precision() * delta_secs;
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

        if linear_velocity.x.abs() < 0.05 {
            linear_velocity.x = 0.0;
        }
    }
}

fn move_player(
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut query: Query<
        (
            Entity,
            &GroundDetection,
            &Collider,
            &mut Transform,
            &mut LinearVelocity,
        ),
        With<CharacterController>,
    >,
) {
    for (entity, ground_detection, collider, mut transform, mut linear_velocity) in &mut query {
        let mut hit_ground_or_ceiling = false;
        let up = transform.up().xy().adjust_precision();

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
            |hit| {
                let angle = up.angle_to(hit.normal.adjust_precision()).abs();
                let is_ground = angle <= ground_detection.max_angle;
                let is_ceiling = angle >= PI - ground_detection.max_angle;

                let [horizontal_component, vertical_component] =
                    split_into_components(linear_velocity.0, up);

                let horizontal_velocity_decomposition =
                    decompose_hit_velocity(horizontal_component, *hit.normal);
                let decomposition = decompose_hit_velocity(*hit.velocity, *hit.normal);

                let slipping_intent =
                    up.dot(horizontal_velocity_decomposition.tangent_part) < -0.001;
                let slipping = up.dot(decomposition.tangent_part) < -0.001;
                let climbing_intent = up.dot(vertical_component) > 0.0;
                let climbing = up.dot(decomposition.tangent_part) > 0.0;

                let projected_velocity = if !is_ground && climbing && !climbing_intent {
                    decomposition.normal_part
                } else if is_ground && slipping && !slipping_intent {
                    decomposition.normal_part
                } else {
                    decomposition.normal_part + decomposition.tangent_part
                };

                *hit.velocity = projected_velocity;

                if is_ground || is_ceiling {
                    hit_ground_or_ceiling = true;
                }

                MoveAndSlideHitResponse::Accept
            },
        );

        transform.translation = position.f32().extend(transform.translation.z);

        if hit_ground_or_ceiling {
            let velocity_along_up = linear_velocity.dot(up);
            let new_velocity_along_up = projected_velocity.dot(up);
            linear_velocity.0 += (new_velocity_along_up - velocity_along_up) * up;
        } else {
            linear_velocity.0 = projected_velocity;
        }
    }
}

fn clear_jump_queue(mut input_state: ResMut<PlayerInputState>) {
    input_state.jump_queued = false;
}

#[derive(Debug)]
struct VelocityDecomposition {
    normal_part: Vector,
    tangent_part: Vector,
}

fn decompose_hit_velocity(velocity: Vector, normal: Dir) -> VelocityDecomposition {
    let normal = normal.adjust_precision();
    let normal_part = normal * normal.dot(velocity);
    let tangent_part = velocity - normal_part;

    VelocityDecomposition {
        normal_part,
        tangent_part,
    }
}

fn split_into_components(v: Vector, up: Vector) -> [Vector; 2] {
    let vertical_component = up * v.dot(up);
    let horizontal_component = v - vertical_component;
    [horizontal_component, vertical_component]
}
