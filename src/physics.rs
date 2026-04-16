use avian2d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(1.0),
        PhysicsDebugPlugin::default(),
    ))
    .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 18.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
