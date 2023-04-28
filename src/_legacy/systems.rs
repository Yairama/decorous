use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;

use rfd;
use dxf::Drawing;
use dxf::entities::*;

use super::resources::*;
use super::components::*;

pub fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn configure_ui_state_system(mut ui_state: ResMut<UIState>) {
    ui_state.is_window_open = true;
}



pub fn ui_example_system(
    mut ui_state: ResMut<UIState>,
    mut is_initialized: Local<bool>,
    mut contexts: EguiContexts,
    mut dxf_files_query: Query<&mut DxfFile>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let ctx = contexts.ctx_mut();
    
    egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File Loader", |ui| {
                if ui.button("Load DXF").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("CAD files (dxf)", &["dxf"]).pick_file() {
                        let dxf = DxfFile{path: Some(path.display().to_string()).unwrap(), visible: true, is_drawn: false};
                        println!("opened DXF file {}", dxf.name());
                        commands.spawn(dxf);
                        
                    }
                }

                if ui.button("Load ASSAY").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("CSV files Assay", &["csv"]).pick_file() {
                        let csv_assay: CsvFile = CsvFile{path: Some(path.display().to_string()).unwrap(), visible: true};
                        println!("opened Assay file {}", csv_assay.name());
                        commands.spawn(csv_assay);   
                    }
                }

                if ui.button("Exit").clicked() {
                    println!("Exit clicked");
                }
            });

        });
    });

    egui::TopBottomPanel::bottom("bottom_menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {

            ui.label("FPS: ");
        });
    });


    egui::SidePanel::left("side_panel")
        .min_width(200.0)
        .max_width(400.0)
        .show(ctx, |ui| {
            

            for mut dxf in dxf_files_query.iter_mut(){
                let txt = dxf.name();
                ui.checkbox(&mut dxf.visible, txt);
            }


            if ui.button("Draw!").clicked(){
                
                for mut dxf in dxf_files_query.iter_mut(){
                    let visible = dxf.visible;
                    let drawn = dxf.is_drawn;
                    if visible && !drawn  {
                        let mut _points : Vec<[f64;3]> = Vec::new();
                        let path = dxf.path();
                        let drawing = Drawing::load_file(&path).unwrap();
                        for e in drawing.entities() {
                            match e.specific {
                                // EntityType::Line(ref _line) => {
                                //     let p1 = _line.p1.clone();
                                //     _points.push([p1.x, p1.y, p1.z]);

                                //     let p2 = _line.p2.clone();
                                //     _points.push([p2.x, p2.y, p2.z]);
                                // },
                                EntityType::LwPolyline(ref _lw_polyline) => {
                                    let vertices = &_lw_polyline.vertices;
                                    let z = _lw_polyline.elevation;
                                    for point in vertices{
                                        _points.push([point.x, point.y, z]);
                                    }
                                },
                                EntityType::Polyline(ref p_line) => {
                                    let vertices = p_line.vertices();
                                    for ver in vertices{
                                        let p = ver.location.clone();
                                        _points.push([p.x, p.y, p.z]);
                                    }
                                },

                                _ => (),
                            }
                        }

                        println!("Total points: {}", _points.len());

                        let (mesh, topo): (Mesh, Topography) = Topography::from_dxf(_points);

                        commands.spawn((PbrBundle{
                            mesh: meshes.add(mesh),
                            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                            ..Default::default()
                        }, topo));
                        dxf.is_drawn = true;
                    }
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(""));
            });

            
    });


}
