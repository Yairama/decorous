use bevy::{prelude::*, pbr::wireframe::WireframePlugin};
use bevy_flycam::{NoCameraPlayerPlugin, MovementSettings};
use bevy_infinite_grid::InfiniteGridPlugin;

mod systems;
mod ui;

use systems::*;
use ui::UIPlugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 1200.0, // default: 12.0
        })
        .add_plugin(WireframePlugin)
        .add_startup_system(setup_system)
        .run();
}

