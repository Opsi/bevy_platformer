use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod game;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            game::plugin,
            player::plugin,
            physics::plugin,
        ))
        .run();
}
