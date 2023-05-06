use std::error::Error;
use std::fs;
use bevy::{
    prelude::*,
    reflect::TypeRegistryInternal,
};
use bevy::app::AppLabel;
use bevy::prelude::{Entity, With};
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use bevy_inspector_egui::egui;
use egui::{Button, RichText, widgets};
use crate::custom_meshes::topography_mesh::TopographyMesh;
use crate::custom_meshes::drill_holes_mesh::DrillHolesMesh;
use crate::ui::ui_file_loader::files::CsvFile;


#[derive(Default)]
pub struct LoadDrillsWindowState{
    assays: String,
    assays_headers: bool,
    header: String,
    header_headers: bool,
    lithography: String,
    lithography_headers: bool,
    survey: String,
    survey_headers: bool,
    topography_mesh: String,
    load_files_result: Option<Result<(), Box<dyn Error + Send + Sync>>>,
}

pub struct LoadDrills;

impl EditorWindow for LoadDrills {

    type State = LoadDrillsWindowState;
    const MENU_BAR : MenuBarWindow = MenuBarWindow::File;
    // const DEFAULT_SIZE: (f32, f32) = (500.0, 500.0);
    const NAME: &'static str = "Load Drills";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui){
        let state = cx.state_mut::<LoadDrills>().unwrap();


        ui.vertical(|ui|{

            ui.horizontal(|ui|{
                egui::TextEdit::singleline(&mut state.assays)
                    .hint_text("HOLE-ID, FROM, TO, AU, CU")
                    .show(ui);

                ui.checkbox( &mut state.assays_headers, "Has headers");

                if ui.button("Load Assay").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Assay", &["csv"]).pick_file() {
                        state.assays = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui|{
                egui::TextEdit::singleline(&mut state.header)
                    .hint_text("HOLE-ID, X, Y, Z, LENGTH")
                    .show(ui);

                ui.checkbox( &mut state.header_headers, "Has headers");

                if ui.button("Load Header").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Header", &["csv"]).pick_file() {
                        state.header = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui|{
                egui::TextEdit::singleline(&mut state.lithography)
                    .hint_text("HOLE-ID, FROM, TO, ROCK")
                    .show(ui);

                ui.checkbox( &mut state.lithography_headers, "Has headers");

                if ui.button("Load Lithography").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Lithography", &["csv"]).pick_file() {
                        state.lithography = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui|{
                egui::TextEdit::singleline(&mut state.survey)
                    .hint_text("HOLE-ID, FROM, TO, AZIMUTH, DIP")
                    .show(ui);

                ui.checkbox( &mut state.survey_headers, "Has headers");

                if ui.button("Load Survey").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Survey", &["csv"]).pick_file() {
                        state.survey = path.display().to_string();
                    }
                }
            });
            ui.label("Select Topography that will be linked to the drill holes: ");
            ui.horizontal(|ui|{
                let mut filtered_query = world
                    .query_filtered::<Entity, (With<Name>, With<TopographyMesh>)>();

                for entity in filtered_query.iter(world){
                    let name = world.get::<Name>(entity).unwrap().to_string();
                    let selected = state.topography_mesh==name;
                    if ui.selectable_label(selected,&name).clicked(){
                        state.topography_mesh = name;
                    }
                }

            });

            let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

            if state.topography_mesh == ""{
                ui.label(RichText::new("No topography selected").color(egui::Color32::RED));
            }

            ui.separator();

            if ui.button("Load Files").clicked() || enter_pressed {
                state.load_files_result = Some(load_files(world, state));
            }

        });

        if let Some(status) = &state.load_files_result {
            match status {
                Ok(()) => {
                    ui.label(RichText::new("Files Loaded!").color(egui::Color32::GREEN));
                }
                Err(error) => {
                    ui.label(RichText::new(error.to_string()).color(egui::Color32::RED));
                }
            }
        }
    }
}

fn load_files(
    world: &mut World,
    state: &mut LoadDrillsWindowState
) -> Result<(), Box<dyn Error + Send + Sync>> {


    let assays_contents = CsvFile{
        path: state.assays.to_string(),
        header: state.assays_headers,
        sep: b',',
    };
    let header_contents = CsvFile{
        path: state.header.to_string(),
        header: state.header_headers,
        sep: b',',
    };
    let lithography_contents = CsvFile{
        path: state.lithography.to_string(),
        header: state.lithography_headers,
        sep: b',',
    };
    let survey_contents = CsvFile{
        path: state.survey.to_string(),
        header: state.survey_headers,
        sep: b',',
    };

    let mut drill_holes = DrillHolesMesh{
        files: [assays_contents, header_contents, lithography_contents, survey_contents],
        offset_x: None,
        offset_y: None,
        offset_z: None,
    };

    if state.topography_mesh!="" {

        let mut filtered_query = world
            .query_filtered::<Entity, (With<Name>, With<TopographyMesh>)>();
        for entity in filtered_query.iter(world){
            let name = world.get::<Name>(entity).unwrap().to_string();
            if name==state.topography_mesh{
                let topography = world.get::<TopographyMesh>(entity).unwrap();
                drill_holes.offset_x = Option::from(topography.offset_x as f32);
                drill_holes.offset_y = Option::from(topography.offset_y as f32);
                drill_holes.offset_z = Option::from(topography.offset_z as f32);
            }
        }
    }

    let final_mesh = DrillHolesMesh::from_csv(drill_holes);

    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = meshes.add(final_mesh);

    let mut materials = world
        .get_resource_mut::<Assets<StandardMaterial>>()
        .unwrap();
    let material = materials.add(
        StandardMaterial::default()
    );

    world.spawn((PbrBundle {
        mesh,
        material,
        ..Default::default()
    },
        if state.topography_mesh=="" {Name::new("Drill Holes")} else {Name::new(state.topography_mesh.clone() + " Drill Holes")}
    ));

    //TODO

    Ok(())
}

