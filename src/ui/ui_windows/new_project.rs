use bevy::{
    prelude::{World},
};
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use bevy_inspector_egui::egui;


pub struct NewProject;

impl EditorWindow for NewProject {

    type State = ();
    const MENU_BAR : MenuBarWindow = MenuBarWindow::File;
    const DEFAULT_SIZE: (f32, f32) = (500.0, 500.0);
    const NAME: &'static str = "New Project";

    fn ui(_world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui){
        
        ui.horizontal(|ui|{
            ui.label("text");

        });


    }



}
