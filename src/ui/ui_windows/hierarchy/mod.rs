// pub mod picking;

use bevy::ecs::entity::Entities;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryInternal;
use bevy::render::{Extract, RenderApp};
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::bevy_inspector::guess_entity_name;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::egui::{self, ScrollArea};
use crate::custom_meshes::components::TopographyMesh;

use crate::ui::ui_core::{
    editor_window::{EditorWindow, EditorWindowContext},
    Editor,
};
use crate::ui::ui_file_loader::files::{CsvFile, DxfFile, FileProperties};
use crate::ui::ui_windows::add::AddItem;
use crate::ui::ui_windows::load_drills::LoadDrills;
use crate::ui::ui_windows::new_project::NewProject;
// use bevy_mod_picking::backends::egui::EguiPointer;
// use bevy_mod_picking::prelude::{IsPointerEvent, PointerClick, PointerButton};

use super::add::{add_ui, AddWindow, AddWindowState};
use super::debug_settings::DebugSettingsWindow;
use super::inspector::{InspectorSelection, InspectorWindow};

#[derive(Component)]
pub struct HideInEditor;

pub struct HierarchyWindow;
impl EditorWindow for HierarchyWindow {
    type State = HierarchyState;
    const NAME: &'static str = "Hierarchy";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {

        hierarchy_menu_bar(ui, world, &mut cx);

        let (hierarchy_state, inspector_state, add_state) =
            match cx.state_mut_triplet::<HierarchyWindow, InspectorWindow, AddWindow>() {
                Some((a, b, c)) => (a, b, Some(c)),
                None => {
                    let (a, b) = cx
                        .state_mut_pair::<HierarchyWindow, InspectorWindow>()
                        .unwrap();
                    (a, b, None)
                }
            };



        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let type_registry = world.resource::<AppTypeRegistry>().clone();
                let type_registry = type_registry.read();
                let new_selected = Hierarchy {
                    world,
                    state: hierarchy_state,
                    type_registry: &type_registry,
                    add_state: add_state.as_deref(),
                }
                .show(ui);

                if new_selected {
                    inspector_state.selected = InspectorSelection::Entities;
                }


            });


    }

    fn app_setup(app: &mut bevy::prelude::App) {
        // picking::setup(app);
        app.add_system(clear_removed_entites.in_base_set(CoreSet::PostUpdate));
        // .add_system(handle_events);

        app.sub_app_mut(RenderApp)
            .add_system(extract_wireframe_for_selected.in_schedule(ExtractSchedule));
    }
}

fn hierarchy_menu_bar<'a>(ui: &mut egui::Ui, world: &mut World, cx: &mut EditorWindowContext){
    ui.separator();

    egui::menu::bar(ui, |ui| {

        ui.separator();

        egui::menu::menu_button(ui, "+", |ui| {
            let response_topography = ui.menu_button("Topography", |ui|{

                ui.menu_button("Poly line", |ui|{
                    if ui.button("DXF").clicked() {
                        // TODO
                    }
                });

                ui.menu_button("Mesh", |ui|{
                    if ui.button("DXF").clicked() {

                        if let Some(path) = rfd::FileDialog::new().add_filter("CAD files (dxf)", &["dxf"]).pick_file() {

                            let dxf = DxfFile{
                                path: Some(path.display().to_string()).unwrap()
                            };

                            let _points: Vec<[f64;3]> = dxf.get_points();
                            let (topography_mesh, topography) = TopographyMesh::from_points(_points);

                            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
                            let mesh = meshes.add(topography_mesh);

                            let mut materials = world
                                .get_resource_mut::<Assets<StandardMaterial>>()
                                .unwrap();
                            let material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

                            world.spawn((PbrBundle {
                                mesh,
                                material,
                                transform: Transform::from_xyz(0.,0.,0.),
                                ..Default::default()
                            }, topography, dxf.clone(), Name::new(dxf.name().unwrap())));

                            ui.close_menu();
                        }
                    }

                    if ui.button("CSV").clicked() {

                        if let Some(path) = rfd::FileDialog::new().add_filter("CAD files (csv)", &["csv"]).pick_file() {
                            let csv = CsvFile{
                                path: Some(path.display().to_string()).unwrap(),
                                header: true,
                                sep: b',',
                            };
                            let (topography_mesh, topography) = TopographyMesh::from_csv(&csv).unwrap();

                            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
                            let mesh = meshes.add(topography_mesh);

                            let mut materials = world
                                .get_resource_mut::<Assets<StandardMaterial>>()
                                .unwrap();
                            let material = materials.add(Color::rgb(0.3, 0.5, 0.3).into());

                            world.spawn((PbrBundle {
                                mesh,
                                material,
                                transform: Transform::from_xyz(0.,0.,0.),
                                ..Default::default()
                            }, topography, csv.clone(), Name::new(csv.name().unwrap())));

                            ui.close_menu();



                        }

                        // TODO
                    }
                });

            });

            if ui.button("Drills").clicked() {

                cx.open_floating_window::<LoadDrills>();
                ui.close_menu();
                // TODO
            }

            ui.menu_button("3D Models", |ui|{
                if ui.button("OBJ").clicked(){
                    // TODO
                }
                if ui.button("gLTF").clicked(){
                    // TODO
                }
            });
        });

        ui.separator();

        egui::menu::menu_button(ui, "↻", |ui|{

        });

    });

    ui.separator();
}


fn clear_removed_entites(mut editor: ResMut<Editor>, entities: &Entities) {
    let state = editor.window_state_mut::<HierarchyWindow>().unwrap();
    state.selected.retain(|entity| entities.contains(entity));
}

/*fn handle_events(
    mut click_events: EventReader<PointerClick>,
    mut editor: ResMut<Editor>,
    editor_state: Res<EditorState>,
    input: Res<Input<KeyCode>>,
    egui_entity: Query<&EguiPointer>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    for click in click_events.iter() {
        if !editor_state.active {
            return;
        }

        if click.event_data().button != PointerButton::Primary {
            continue;
        }

        if egui_entity.get(click.target()).is_ok() || egui_ctx.ctx_mut().wants_pointer_input() {
            continue;
        };

        let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

        let ctrl = input.any_pressed([KeyCode::LControl, KeyCode::RControl]);
        let shift = input.any_pressed([KeyCode::LShift, KeyCode::RShift]);
        let mode = SelectionMode::from_ctrl_shift(ctrl, shift);

        let entity = click.target();
        info!("Selecting mesh, found {:?}", entity);
        state
            .selected
            .select(mode, entity, |_, _| std::iter::once(entity));
    }
}*/

fn extract_wireframe_for_selected(editor: Extract<Res<Editor>>, mut commands: Commands) {
    let wireframe_for_selected = editor
        .window_state::<DebugSettingsWindow>()
        .map_or(false, |settings| settings.highlight_selected);

    if wireframe_for_selected {
        let selected = &editor.window_state::<HierarchyWindow>().unwrap().selected;
        for selected in selected.iter() {
            commands.get_or_spawn(selected).insert(Wireframe);
        }
    }
}

#[derive(Default)]
pub struct HierarchyState {
    pub selected: SelectedEntities,
    rename_info: Option<RenameInfo>,
}

pub struct RenameInfo {
    entity: Entity,
    renaming: bool,
    current_rename: String,
}

struct Hierarchy<'a> {
    world: &'a mut World,
    state: &'a mut HierarchyState,
    type_registry: &'a TypeRegistryInternal,
    add_state: Option<&'a AddWindowState>,
}

impl<'a> Hierarchy<'a> {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut despawn_recursive = None;
        let mut despawn = None;

        let HierarchyState {
            selected,
            rename_info,
        } = self.state;

        let new_selection = bevy_inspector_egui::bevy_inspector::hierarchy::Hierarchy {
            extra_state: rename_info,
            world: self.world,
            type_registry: self.type_registry,
            selected,
            context_menu: Some(&mut |ui, entity, world, rename_info| {
                if ui.button("Despawn").clicked() {
                    despawn_recursive = Some(entity);
                }

                if ui.button("Remove keeping children").clicked() {
                    despawn = Some(entity);
                }

                if ui.button("Rename").clicked() {
                    let entity_name = guess_entity_name(world, entity);
                    *rename_info = Some(RenameInfo {
                        entity,
                        renaming: true,
                        current_rename: entity_name,
                    });
                    ui.close_menu();
                }

                if let Some(add_state) = self.add_state {
                    ui.menu_button("Add", |ui| {
                        if let Some(add_item) = add_ui(ui, add_state) {
                            add_item.add_to_entity(world, entity);
                            ui.close_menu();
                        }
                    });
                }
            }),
            shortcircuit_entity: Some(&mut |ui, entity, world, rename_info| {
                if let Some(rename_info) = rename_info {
                    if rename_info.renaming && rename_info.entity == entity {
                        rename_entity_ui(ui, rename_info, world);

                        return true;
                    }
                }

                false
            }),
        }
        .show::<Without<HideInEditor>>(ui);

        if let Some(entity) = despawn_recursive {
            bevy::hierarchy::despawn_with_children_recursive(self.world, entity);
        }
        if let Some(entity) = despawn {
            self.world.entity_mut(entity).despawn();
            self.state.selected.remove(entity);
        }

        if ui.input(|input| input.key_pressed(egui::Key::Delete)) {
            for entity in self.state.selected.iter() {
                self.world.entity_mut(entity).despawn_recursive();
            }
            self.state.selected.clear();
        }

        new_selection
    }
}

fn rename_entity_ui(ui: &mut egui::Ui, rename_info: &mut RenameInfo, world: &mut World) {
    use egui::epaint::text::cursor::CCursor;
    use egui::widgets::text_edit::{CCursorRange, TextEdit, TextEditOutput};

    let id = egui::Id::new(rename_info.entity);

    let edit = TextEdit::singleline(&mut rename_info.current_rename).id(id);
    let TextEditOutput {
        response,
        state: mut edit_state,
        ..
    } = edit.show(ui);

    // Runs once to end renaming
    if response.lost_focus() {
        rename_info.renaming = false;

        match world.get_entity_mut(rename_info.entity) {
            Some(mut ent_mut) => match ent_mut.get_mut::<Name>() {
                Some(mut name) => {
                    name.set(rename_info.current_rename.clone());
                }
                None => {
                    ent_mut.insert(Name::new(rename_info.current_rename.clone()));
                }
            },
            None => {
                error!("Failed to get renamed entity");
            }
        }
    }

    // Runs once when renaming begins
    if !response.has_focus() {
        response.request_focus();

        edit_state.set_ccursor_range(Some(CCursorRange::two(
            CCursor::new(0),
            CCursor::new(rename_info.current_rename.len()),
        )));
    }

    TextEdit::store_state(ui.ctx(), id, edit_state);
}
