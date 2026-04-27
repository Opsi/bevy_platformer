use bevy::{
    color::palettes::tailwind::{CYAN_500, ORANGE_500},
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
pub struct PlayerFeetCollider;

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
        let player_feet = Rectangle::new(60., 40.);
        parent.spawn((
            Name::new("PlayerFeetCollider"),
            PlayerFeetCollider,
            Mesh2d(meshes.add(player_feet.to_ring(5.))),
            MeshMaterial2d(materials.add(Color::Srgba(ORANGE_500))),
            Transform::from_xyz(0., player_feet.half_size.y, -0.5),
        ));

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
        ));
    });
}
