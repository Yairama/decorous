use bevy::app::App;
use bevy::prelude::World;
use egui::{Ui};
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};

#[derive(Default)]
pub struct NodesCreatorState{
    search: String,
    scene_save_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

pub struct NodesCreator;

impl EditorWindow for NodesCreator {
    type State = NodesCreatorState;
    const NAME: &'static str = "Create Node";
    const RESIZABLE: bool = false;
    const DEFAULT_SIZE: (f32, f32) = (700.0, 500.0);
    const COLLAPSIBLE: bool = false;
    const MENU_BAR: MenuBarWindow = MenuBarWindow::File;

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut Ui) {
        let state = cx.state_mut::<NodesCreator>().unwrap();

        ui.horizontal(|ui|{
            egui::ScrollArea::vertical()
                .max_width(200.0)
                .show(ui,|ui|{
                ui.heading("\u{2605} Favourites");
            });
            ui.separator();
            ui.vertical(|ui|{
                ui.label("Search:");
                let response = egui::TextEdit::singleline(&mut state.search).show(ui);
                if response.response.changed() {
                    state.scene_save_result = None;
                }
                ui.separator();
                egui::ScrollArea::vertical()
                    .max_width(300.)
                    .max_height(500.)
                    .min_scrolled_height(500.)
                    .min_scrolled_width(300.)
                    .show(ui, |ui|{

                        egui::CollapsingHeader::new("\u{2B55} Node")
                            .default_open(true)
                            .show(ui, |ui|{
                                egui::CollapsingHeader::new("\u{1F5FA} Topography Mesh")
                                    .default_open(true)
                                    .show(ui, |ui|{
                                        if ui.button("\u{1F5B9} From dxf file").clicked(){
                                            //TODO
                                        }
                                        if ui.button("\u{1F5B9} From csv file").clicked(){
                                            //TODO
                                        }
                                    });
                                let drill_holes_res = ui.selectable_label(false,
                                                                          "\u{1F4A2} Drill Holes");
                                if drill_holes_res.clicked(){
                                    println!("Calla cagada");
                                }



                            });



                });
            });

        });



    }

}