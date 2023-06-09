use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_mod_picking::{PickableBundle};
use bevy_mod_picking::prelude::*;
use bevy_egui::egui;

use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext};
use crate::ui::ui_windows::cameras::camera_3d_panorbit::CameraSystem;

/// Prevents the entity from being selectable in the editor window.
#[derive(Component)]
pub struct NoEditorPicking;

pub struct PickingWindow;

impl EditorWindow for PickingWindow {
    type State = ();
    const NAME: &'static str = "PickingWindow";

    fn ui(_world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        ui.label("OAAAAAAAAAAAA");
    }

    fn app_setup(app: &mut App) {
        setup(app);
    }
}


pub fn setup(app: &mut App) {
    app
        // .add_plugins(DefaultPickingPlugins)
        .add_system(auto_add_editor_picking_set.in_set(CameraSystem::Movement));
}

fn auto_add_editor_picking_set(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    meshes_query: Query<
        (Entity, &Handle<Mesh>),
        (Without<RaycastPickTarget>, Without<NoEditorPicking>),
    >,
) {
    for (entity, handle) in meshes_query.iter() {
        if let Some(mesh) = meshes.get(handle) {
            if let PrimitiveTopology::TriangleList = mesh.primitive_topology() {
                commands
                    .entity(entity)
                    .insert((PickableBundle::default(), RaycastPickTarget::default()));
            }
        }
    }
}
