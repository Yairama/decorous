use std::fmt::Error;
use std::fs;
use bevy::{
    prelude::{AppTypeRegistry, ReflectResource, World},
    reflect::TypeRegistryInternal,
};
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use bevy_inspector_egui::egui;
use egui::{Button, RichText, widgets};
use crate::ui::ui_file_loader::files::{AssayFile, FileProperties, HeaderFile, LithographyFile, SurveyFile};

#[derive(Default)]
pub struct LoadDrillsWindowState{
    assays: String,
    header: String,
    lithography: String,
    survey: String,
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
                let response_assay = egui::TextEdit::singleline(&mut state.assays)
                    .hint_text("assay.csv")
                    .show(ui);

                if ui.button("Load Assay").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Assay", &["csv","txt"]).pick_file() {
                        state.assays = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui|{
                let response_header = egui::TextEdit::singleline(&mut state.header)
                    .hint_text("header.csv")
                    .show(ui);

                if ui.button("Load Header").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Header", &["csv","txt"]).pick_file() {
                        state.header = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui|{
                let response_lithography = egui::TextEdit::singleline(&mut state.lithography)
                    .hint_text("lithography.csv")
                    .show(ui);

                if ui.button("Load Lithography").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Lithography", &["csv","txt"]).pick_file() {
                        state.lithography = path.display().to_string();
                    }
                }
            });

            ui.horizontal(|ui|{
                let response_survey = egui::TextEdit::singleline(&mut state.survey)
                    .hint_text("survey.csv")
                    .show(ui);

                if ui.button("Load Survey").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Survey", &["csv","txt"]).pick_file() {
                        state.survey = path.display().to_string();
                    }
                }
            });

            let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

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

    let assay_component = AssayFile{path: String::from(&state.assays)};
    let header_component = HeaderFile{path: String::from(&state.header)};
    let lithography_component = LithographyFile{path: String::from(&state.lithography)};
    let survey_component = SurveyFile{path: String::from(&state.survey)};

    println!("{:?}", assay_component.name_with_extension());

    //TODO

    Ok(())
}

