use std::error::Error;
use bevy::app::App;
use bevy::prelude::World;
use bevy_egui::egui;
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

    fn ui(_world: &mut World, _cx: EditorWindowContext, _ui: &mut Ui) {
        todo!()
    }

    fn menu_ui(_world: &mut World, _cx: EditorWindowContext, _ui: &mut Ui) {
        todo!()
    }

    fn viewport_toolbar_ui(_world: &mut World, _cx: EditorWindowContext, _ui: &mut Ui) {
        todo!()
    }

    fn viewport_ui(_world: &mut World, _cx: EditorWindowContext, _ui: &mut Ui) {
        todo!()
    }

    fn app_setup(_app: &mut App) {
        todo!()
    }
}