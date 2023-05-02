use std::fmt::Error;
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
use crate::custom_meshes::components::TopographyMesh;


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
    load_files_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
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
    world: &World,
    state: &mut LoadDrillsWindowState
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    let assays_contents = fs::read_to_string(&state.assays)?;
    let header_contents = fs::read_to_string(&state.header)?;
    let lithography_contents = fs::read_to_string(&state.lithography)?;
    let survey_contents = fs::read_to_string(&state.survey)?;

    // let assay_component = AssayFile{path: String::from(&state.assays)};
    // let header_component = HeaderFile{path: String::from(&state.header)};
    // let lithography_component = LithographyFile{path: String::from(&state.lithography)};
    // let survey_component = SurveyFile{path: String::from(&state.survey)};

    //TODO

    Ok(())
}

