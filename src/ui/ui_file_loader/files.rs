use std::path::Path;
use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology};
use delaunator::{Point, triangulate};
use dxf::Drawing;
use dxf::entities::EntityType;

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
            .map(|nombre| nombre.to_string())
    }


}


pub trait DxfEntitiesManager{

    fn path(&self) -> String;

    fn get_points(&self) -> Vec<[f64;3]>{
        let mut _points : Vec<[f64;3]> = Vec::new();
        let path = self.path();
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
pub struct DxfFile{
    pub path: String
}

impl FileProperties for DxfFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl DxfEntitiesManager for DxfFile {
    fn path(&self) -> String {
        self.path.clone()
    }
}


#[derive(Component)]
pub struct TopographyMesh{
    min_x: f64,
    min_y: f64,
    min_z: f64,
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

    pub fn from_points(mut vec: Vec<[f64;3]>) -> (Mesh, Self){
        let min_x = vec.iter().map(|v| v[0]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_y = vec.iter().map(|v| v[1]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_z = vec.iter().map(|v| v[2]).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        for v in vec.iter_mut() {
            v[0] -= min_x;
            v[1] -= min_y;
            v[2] -= min_z;
        };

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
        println!("poss {:?}",vec[100]);
        (mesh, Self { min_x:min_x, min_y: min_y, min_z: min_z })

    }
}


#[derive(Component)]
pub struct AssayFile{
    pub path: String
}

impl FileProperties for AssayFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Component)]
pub struct HeaderFile{
    pub path: String
}

impl FileProperties for HeaderFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Component)]
pub struct LithographyFile{
    pub path: String
}

impl FileProperties for LithographyFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Component)]
pub struct SurveyFile{
    pub path: String
}

impl FileProperties for SurveyFile{
    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Component)]
pub struct DrillHole;



