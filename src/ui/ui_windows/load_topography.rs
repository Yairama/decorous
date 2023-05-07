use std::error::Error;
use bevy::app::App;
use bevy::prelude::World;
use egui::Ui;
use crate::ui::ui_core::editor_window::{EditorWindowContext, MenuBarWindow};
use crate::ui::ui_setup::editor_window::EditorWindow;

#[derive(Default)]
pub struct LoadTopographyState{
    topography: String,
    load_files_result: Option<Result<(), Box<dyn Error + Send + Sync>>>,
}

pub struct LoadTopography;

impl EditorWindow for LoadTopography {
    type State = LoadTopographyState;
    const NAME: &'static str = "Load Topography";
    const MENU_BAR: MenuBarWindow = MenuBarWindow::File;

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut Ui) {
        todo!()
    }

    fn menu_ui(world: &mut World, cx: EditorWindowContext, ui: &mut Ui) {
        todo!()
    }

    fn viewport_toolbar_ui(world: &mut World, cx: EditorWindowContext, ui: &mut Ui) {
        todo!()
    }

    fn viewport_ui(world: &mut World, cx: EditorWindowContext, ui: &mut Ui) {
        todo!()
    }

    fn app_setup(app: &mut App) {
        todo!()
    }
}