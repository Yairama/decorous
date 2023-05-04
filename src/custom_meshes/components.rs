use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use bevy::prelude::*;
use bevy::prelude::shape::Cylinder;
use bevy::prelude::system_adapter::new;
use bevy::reflect::erased_serde::__private::serde::__private::de::Content::String;
use delaunator::{Point, triangulate};
use bevy::render::mesh::{PrimitiveTopology};
use bevy::render::render_resource::Face;
use csv::ReaderBuilder;
use polars::export::arrow::array::equal;
use polars::prelude::*;
use crate::ui::ui_file_loader::files::{CsvFile};
use crate::utilities::math::analytic_geometry;

#[derive(Component)]
pub struct TopographyMesh{
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
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
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let result = triangulate(&points);

        let triangles = result.triangles;
        let vector_values = vec.iter().map(|v| Vec3::new(v[0] as f32, v[2] as f32, v[1] as f32)).collect::<Vec<_>>();
        let normals = Self::calculate_normals(&vector_values, &triangles);

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; vector_values.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vector_values);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(triangles.into_iter().map(|i| i as u32).collect())));

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

/// Saves the files
/// 0: Assay
/// 1: Header
/// 2: Lithography
/// 3: Survey
#[derive(Component)]
pub struct DrillHolesMesh{
    pub files: [CsvFile;4],
    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,
    pub offset_z: Option<f32>,
}

impl DrillHolesMesh {
    pub fn from_csv(drill_holes: DrillHolesMesh) -> (Vec<Mesh>,Vec<Vec3>, Vec<StandardMaterial>){
        let assay = &drill_holes.files[0];
        let header = &drill_holes.files[1];
        let survey = &drill_holes.files[3];


        let df_assay = assay.dataframe().unwrap();
        let mut df_header = header.dataframe().unwrap();
        let df_survey = survey.dataframe().unwrap();

        let mut meshes_result: Vec<Mesh> = Vec::new();
        let mut transforms_result: Vec<Vec3> = Vec::new();
        let mut material_au_result: Vec<StandardMaterial> = Vec::new();

        let p25_grade_au = df_assay.column("au").unwrap().f64().unwrap()
            .quantile(0.25, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;
        let p75_grade_au = df_assay.column("au").unwrap().f64().unwrap()
            .quantile(0.75, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;
        let p25_grade_cu = df_assay.column("cu").unwrap().f64().unwrap()
            .quantile(0.25, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;
        let p75_grade_cu = df_assay.column("cu").unwrap().f64().unwrap()
            .quantile(0.75, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;


        let x_header_colum = df_header.column("x").unwrap().sub(drill_holes.offset_x.unwrap());
        df_header = (*df_header.with_column(x_header_colum).unwrap()).clone();

        let y_header_colum = df_header.column("y").unwrap().sub(drill_holes.offset_y.unwrap());
        df_header = (*df_header.with_column(y_header_colum).unwrap()).clone();

        let z_header_colum = df_header.column("z").unwrap().sub(drill_holes.offset_z.unwrap());
        df_header = (*df_header.with_column(z_header_colum).unwrap()).clone();

        let df_drills_orientation = df_header.left_join(&df_survey, ["hole-id"], ["hole-id"]).unwrap();

        let mut iters_drills = df_drills_orientation
            .columns(["hole-id","x","y","z","from","to","azimuth","dip"]).unwrap()
            .iter().map(|s| s.iter()).collect::<Vec<_>>();




        for _row_drills in 0..df_drills_orientation.height(){
            let hole_id = iters_drills[0].next().unwrap().to_string().replace("\"", "");
            let x = iters_drills[1].next().unwrap().try_extract::<f32>().unwrap();
            let y = iters_drills[2].next().unwrap().try_extract::<f32>().unwrap();
            let z = iters_drills[3].next().unwrap().try_extract::<f32>().unwrap();
            let survey_from = iters_drills[4].next().unwrap().try_extract::<f32>().unwrap();
            let survey_to = iters_drills[5].next().unwrap().try_extract::<f32>().unwrap();
            let azimuth = iters_drills[6].next().unwrap().try_extract::<f32>().unwrap();
            let dip = iters_drills[7].next().unwrap().try_extract::<f32>().unwrap();

            let df_filtered_assays = df_assay.filter(&df_assay
                .column("hole-id").unwrap().utf8().unwrap()
                .contains_literal(&hole_id).unwrap()).unwrap();

            let mut iters_assay = df_filtered_assays
                .columns(["hole-id","from","to","au","cu"]).unwrap()
                .iter().map(|s| s.iter()).collect::<Vec<_>>();

            for _row_assay in 0..df_filtered_assays.height(){

                let _hole_id = iters_assay[0].next().unwrap().to_string().replace("\"", "");
                let from = iters_assay[1].next().unwrap().try_extract::<f32>().unwrap();
                let to = iters_assay[2].next().unwrap().try_extract::<f32>().unwrap();
                let au = iters_assay[3].next().unwrap().try_extract::<f32>().unwrap();
                let cu = iters_assay[4].next().unwrap().try_extract::<f32>().unwrap();

                let from_coord = analytic_geometry::interpolate_point_on_the_line(
                    [x,z,y],
                    azimuth,
                    dip,
                    from
                );

                let to_coord = analytic_geometry::interpolate_point_on_the_line(
                    [x,z,y],
                    azimuth,
                    dip,
                    to
                );

                let mesh = Self::generate_triangular_prisma(
                    &from_coord,
                    &to_coord,
                    5.0);

                let material_grade = StandardMaterial{
                    base_color: Self::material_color_scale((au-p25_grade_au)/(p75_grade_au-p25_grade_au)),
                    // cull_mode: Option::from(Face::Front),
                    ..Default::default()
                };

                meshes_result.push(mesh);
                transforms_result.push((from_coord+to_coord)*0.5);
                material_au_result.push(material_grade);
            }

        }

        //TODO
        (meshes_result, transforms_result, material_au_result)
    }

    fn material_color_scale(value: f32) -> Color {
        let (r, g, b) = if value < 0.5 {
            let v = if value>0.0 {value * 2.0} else {0.0};
            (v, v, 0.0)
        } else {
            let v = if (1.0 - value) > 0.0 {(1.0 - value) * 2.0} else {1.0};
            (v, 1.0, 0.0)
        };
        Color::rgb(r, g, b)
    }

    fn generate_triangular_prisma(
        coord1: &Vec3,
        coord2: &Vec3,
        radius: f32
    ) -> Mesh {
        let length = (coord2.x - coord1.x).hypot(coord2.y - coord1.y).hypot(coord2.z - coord1.z);

        let shape = Cylinder {
            radius: radius,
            height: length,
            resolution: 3,
            ..Default::default()
        };

        Mesh::from(shape)
    }



}

