use bevy::{diagnostic::Diagnostics, prelude::*};
use crate::ui::ui_core::editor_window::{EditorWindow, EditorWindowContext, MenuBarWindow};
use bevy_egui::egui;

pub struct DiagnosticsWindow;
impl EditorWindow for DiagnosticsWindow {
    type State = ();
    const MENU_BAR : MenuBarWindow = MenuBarWindow::About;
    const NAME: &'static str = "Diagnostics";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        let diagnostics = match world.get_resource::<Diagnostics>() {
            Some(diagnostics) => diagnostics,
            None => {
                ui.label("Diagnostics resource not available");
                return;
            }
        };
        diagnostic_ui(ui, diagnostics);
    }
}

fn diagnostic_ui(ui: &mut egui::Ui, diagnostics: &Diagnostics) {
    egui::Grid::new("frame time diagnostics").show(ui, |ui| {
        let mut has_diagnostics = false;
        for diagnostic in diagnostics.iter() {
            has_diagnostics = true;
            ui.label(diagnostic.name.as_ref());
            if let Some(average) = diagnostic.average() {
                ui.label(format!("{:.2}", average));
            }
            ui.end_row();
        }

        if !has_diagnostics {
            ui.label(
                r#"No diagnostics found. Possible plugins to add:
            - `FrameTimeDiagnosticsPlugin`
            - `EntityCountDiagnisticsPlugin`
            - `AssetCountDiagnosticsPlugin`
            "#,
            );
        }
    });
}
