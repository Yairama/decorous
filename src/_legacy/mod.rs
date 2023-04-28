use bevy::prelude::*;

mod resources;
mod systems;
mod components;

use bevy_egui::EguiPlugin;
use resources::*;
use systems::*;


pub struct UIPlugin;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut App){
        app
        .init_resource::<UIState>()
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_visuals_system)
        .add_startup_system(configure_ui_state_system)
        .add_system(ui_example_system);
    }
}