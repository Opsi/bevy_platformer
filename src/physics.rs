use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
