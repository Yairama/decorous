use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;
use delaunator::{Point, triangulate};
use bevy::render::mesh::{PrimitiveTopology};
use csv::ReaderBuilder;
use crate::ui::ui_file_loader::files::CsvFile;

#[derive(Component)]
pub struct TopographyMesh{
    offset_x: f64,
    offset_y: f64,
    offset_z: f64,
}

impl TopographyMesh {
    fn calculate_normals(vertices: &[Vec3], triangles: &[usize]) -> Vec<Vec3> {
        let mut normals = vec![Vec3::ZERO; vertices.len()];
        for chunk in triangles.chunks(3) {
            let a = vertices[chunk[0]];
            let b = vertices[chunk[1]];
            let c = vertices[chunk[2]];
            let normal = (b - a).cross(c - a).normalize();
            normals[chunk[0]] += normal;
            normals[chunk[1]] += normal;
            normals[chunk[2]] += normal;
        }
        for normal in &mut normals {
            *normal = normal.normalize();
        }
        normals
    }

    fn create_mesh(vec: Vec<[f64;3]>) -> Mesh{
        let points = vec.iter().map(|v| Point { x: v[0], y: v[1] }).collect::<Vec<Point>>();

        let result = triangulate(&points);
        let triangles = result.triangles;


        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vector_values = vec.iter().map(|v| Vec3::new(v[0] as f32, v[2] as f32, v[1] as f32)).collect::<Vec<_>>();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vector_values.clone(),
        );

        let normals = Self::calculate_normals(&vector_values, &triangles);

        // In this example, normals and UVs don't matter,
        // so we just use the same value for all of them
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; vec.len()]);
        // A triangle using vertices 0, 2, and 1.
        // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(triangles.clone().into_iter().map(|i| i as u32).collect())));
        mesh
    }

    pub fn from_points(mut vec: Vec<[f64;3]>) -> (Mesh, Self){
        let min_x = vec.iter().map(|v| v[0]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_y = vec.iter().map(|v| v[1]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_z = vec.iter().map(|v| v[2]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        for v in vec.iter_mut() {
            v[0] -= min_x;
            v[1] -= min_y;
            v[2] -= min_z;
        };
        let mesh = Self::create_mesh(vec);

        (mesh, Self { offset_x:min_x, offset_y: min_y, offset_z: min_z })
    }

    pub fn from_csv(csv: &CsvFile) -> Result<(Mesh, Self), Box<dyn Error>>{

        let file = csv.get_file().unwrap();
        let reader = BufReader::new(file);
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(csv.header)
            .delimiter(csv.sep)
            .from_reader(reader);
        let mut coords: Vec<[f64; 3]> = vec![];
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;

        for result in csv_reader.records() {
            let record = result?;

            let x = record[0].parse::<f64>()?;
            let y = record[1].parse::<f64>()?;
            let z = record[2].parse::<f64>()?;

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);

            coords.push([x, y, z]);
        }

        for v in coords.iter_mut() {
            v[0] -= min_x;
            v[1] -= min_y;
            v[2] -= min_z;
        }

        let mesh = Self::create_mesh(coords);
        Ok((mesh, Self { offset_x:min_x, offset_y: min_y, offset_z: min_z }))

    }

}

#[derive(Component)]
pub struct DrillHolesMesh{
    offset_x: f64,
    offset_y: f64,
    offset_z: f64,
}

impl DrillHolesMesh {

}

