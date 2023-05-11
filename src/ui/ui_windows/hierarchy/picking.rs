use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::prelude::{PickableBundle, RaycastPickTarget};
use egui::Ui;
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext};

pub struct EditorPickingSet;

/// Prevents the entity from being selectable in the editor window.
#[derive(Component)]
pub struct NoEditorPicking;

pub struct PickingWindow;

impl EditorWindow for PickingWindow {
    type State = ();
    const NAME: &'static str = "PickingWindow";

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut Ui) {
        ui.label("OAAAAAAAAAAAA");
    }

    fn app_setup(app: &mut App) {
        setup(app);
    }
}

pub fn setup(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins)
        .add_system(auto_add_editor_picking_set);
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
