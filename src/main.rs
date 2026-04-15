use bevy::prelude::*;

mod level;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            player::plugin,
            level::plugin,
            physics::plugin,
        ))
        .run();
}
