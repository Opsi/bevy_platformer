pub mod world;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(world::plugin);
}
