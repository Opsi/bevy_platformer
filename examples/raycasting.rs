use bevy::prelude::*;

fn main() {
    dbg!("running raycaster example");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
