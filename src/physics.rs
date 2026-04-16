use avian2d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};

use crate::player::Player;

#[derive(Component)]
struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(1.0),
        PhysicsDebugPlugin::default(),
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, follow_player);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 18.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
