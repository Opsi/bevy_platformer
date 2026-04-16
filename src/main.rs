use bevy::prelude::*;

mod game;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            game::plugin,
            player::plugin,
            physics::plugin,
        ))
        .run();
}
