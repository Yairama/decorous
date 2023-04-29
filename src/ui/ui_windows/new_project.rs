use bevy::{
    prelude::{AppTypeRegistry, ReflectResource, World},
    reflect::TypeRegistryInternal,
};
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use bevy_inspector_egui::egui;


pub struct NewProject;

impl EditorWindow for NewProject {

    type State = ();
    const MENU_BAR : MenuBarWindow = MenuBarWindow::File;
    const NAME: &'static str = "New Project";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui){
        
        ui.horizontal(|ui|{
            ui.label("text");

            

        });


    }



}
