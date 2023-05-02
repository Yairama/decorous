use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology};
use csv::{Reader, ReaderBuilder};
use delaunator::{Point, triangulate};
use dxf::Drawing;
use dxf::entities::EntityType;
use polars::error::PolarsError;
use polars::prelude::*;

pub trait FileProperties{

    fn path(&self) -> String;

    fn name_with_extension(&self) -> Option<String> {
        let ruta = self.path().clone().to_string();
        let path = Path::new(ruta.as_str());
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name1| name1.to_string())
    }

    fn name(&self) -> Option<String> {
        let ruta = self.path().clone().to_string();
        let path = Path::new(ruta.as_str());
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|name| name.to_string())
    }


}

#[derive(Component, Clone)]
pub struct DxfFile{
    pub path: String
}

impl FileProperties for DxfFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl DxfFile {
    pub fn get_points(&self) -> Vec<[f64;3]>{
        let mut _points : Vec<[f64;3]> = Vec::new();
        let path = self.path.clone();
        let drawing = Drawing::load_file(&path).unwrap();
        for e in drawing.entities() {
            match e.specific {
                EntityType::Line(ref _line) => {
                    let p1 = _line.p1.clone();
                    _points.push([p1.x, p1.y, p1.z]);

                    let p2 = _line.p2.clone();
                    _points.push([p2.x, p2.y, p2.z]);
                },
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
        _points
    }

}

#[derive(Component, Clone)]
pub struct CsvFile{
    pub path: String,
    pub header: bool,
    pub sep: u8
}

impl FileProperties for CsvFile {
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl CsvFile {

    pub fn get_file(&self) -> Result<File, Box<dyn Error>>{
        let mut file = File::open(self.path.clone())?;
        Ok(file)
    }

    fn read_csv_file(&self) -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
        let file = File::open(self.path.clone())?;
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.unwrap());
        Ok(lines)
    }


    fn csv_to_lowercase_dataframe(&self) -> PolarsResult<DataFrame> {
        let file = File::open(self.path.clone())?;

        let mut df = CsvReader::new(file)
            .has_header(self.header)
            .finish()?;

        if self.header {
            // Convierte los encabezados a minúsculas
            let lowercase_columns: Vec<String> = df
                .get_column_names()
                .iter()
                .map(|col_name| col_name.to_lowercase())
                .collect();

            df.set_column_names(&lowercase_columns)?;
        }

        Ok(df)
    }



}

/// Saves the files
/// 0: Assay
/// 1: Header
/// 2: Lithography
/// 3: Survey
#[derive(Component)]
pub struct DrillHole{
    pub files: [CsvFile;4],
}


