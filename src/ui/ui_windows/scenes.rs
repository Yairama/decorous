use bevy::prelude::*;
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use bevy_egui::egui;

const DEFAULT_FILENAME: &str = "scene.scn.ron";

#[derive(Default, Component)]
pub struct NotInScene;

#[derive(Default)]
pub struct SceneWindowState {
    filename: String,
    scene_save_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

pub struct SceneWindow;

impl EditorWindow for SceneWindow {
    type State = SceneWindowState;
    const MENU_BAR : MenuBarWindow = MenuBarWindow::File;
    const NAME: &'static str = "Scenes";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<SceneWindow>().unwrap();

        ui.horizontal(|ui| {
            let res = egui::TextEdit::singleline(&mut state.filename)
                .hint_text(DEFAULT_FILENAME)
                .desired_width(200.0)
                .show(ui);

            if res.response.changed() {
                state.scene_save_result = None;
            }

            let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

            if ui.button("Save").clicked() || enter_pressed {
                let filename = if state.filename.is_empty() {
                    DEFAULT_FILENAME
                } else {
                    &state.filename
                };
                let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
                let entitys = query.iter(world).collect();
                state.scene_save_result = Some(save_world(world, filename, entitys));
            }
        });

        if let Some(status) = &state.scene_save_result {
            match status {
                Ok(()) => {
                    ui.label(egui::RichText::new("Success!").color(egui::Color32::GREEN));
                }
                Err(error) => {
                    ui.label(egui::RichText::new(error.to_string()).color(egui::Color32::RED));
                }
            }
        }
    }
}

fn save_world(
    world: &World,
    name: &str,
    entities: std::collections::HashSet<Entity>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let mut scene_builder =
        DynamicSceneBuilder::from_world_with_type_registry(world, type_registry.clone());
    scene_builder.extract_entities(entities.into_iter());
    let scene = scene_builder.build();

    let ron = scene.serialize_ron(type_registry)?;
    std::fs::write(name, ron)?;
    Ok(())
}
