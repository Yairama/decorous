use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin}};
use bevy_infinite_grid::InfiniteGridPlugin;

mod ui;
mod systems;
mod custom_meshes;
mod utilities;

use custom_meshes::*;
use utilities::*;
use systems::*;
use ui::ui_setup::EditorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(28.0/255.0, 28.0/255.0, 36.0/255.0)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: "Decorous".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EditorPlugin::new())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup_system)
        .run();
}
