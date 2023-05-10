


use std::ops::Sub;
use bevy::prelude::*;
use bevy::prelude::shape::Cylinder;







use polars::prelude::*;
use crate::ui::ui_file_loader::files::{CsvFile};
use crate::utilities::math::analytic_geometry;


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
    pub fn from_csv(drill_holes: DrillHolesMesh) -> Vec<Mesh>{
        let assay = &drill_holes.files[0];
        let header = &drill_holes.files[1];
        let lithography = &drill_holes.files[2];
        let survey = &drill_holes.files[3];



        let df_assay = assay.dataframe().unwrap();
        let mut df_header = header.dataframe().unwrap();
        let df_survey = survey.dataframe().unwrap();
        let df_lithography = lithography.dataframe().unwrap();

        let mut au_grades_meshes_result: Vec<Mesh> = Vec::new();
        let mut cu_grades_meshes_result: Vec<Mesh> = Vec::new();
        let mut transforms_result: Vec<Transform> = Vec::new();
        let _material_au_result: Vec<[f32;3]> = Vec::new();
        let _material_cu_result: Vec<[f32;3]> = Vec::new();
        let _material_lithography: Vec<[f32;3]> = Vec::new();

        let p25_grade_au = df_assay.column("au").unwrap().f64().unwrap()
            .quantile(0.25, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;
        let p75_grade_au = df_assay.column("au").unwrap().f64().unwrap()
            .quantile(0.75, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;

        let p25_grade_cu = df_assay.column("cu").unwrap().f64().unwrap()
            .quantile(0.25, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;
        let p75_grade_cu = df_assay.column("cu").unwrap().f64().unwrap()
            .quantile(0.75, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;

        let p25_lithography = df_lithography.column("rock").unwrap().i64().unwrap()
            .quantile(0.25, QuantileInterpolOptions::Linear).unwrap().unwrap() as f32;
        let p75_lithography = df_lithography.column("rock").unwrap().i64().unwrap()
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
            let _survey_from = iters_drills[4].next().unwrap().try_extract::<f32>().unwrap();
            let _survey_to = iters_drills[5].next().unwrap().try_extract::<f32>().unwrap();
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

                let grade_from_coord = analytic_geometry::interpolate_point_on_the_line(
                    [x,y,z],
                    azimuth,
                    dip,
                    from
                );

                let grade_to_coord = analytic_geometry::interpolate_point_on_the_line(
                    [x,y,z],
                    azimuth,
                    dip,
                    to
                );

                let mut prisma_mesh = Self::generate_triangular_prisma(
                    &grade_from_coord,
                    &grade_to_coord,
                    3.0);


                let material_au_grade = super::mesh_handlers::color_scale((au-p25_grade_au)/(p75_grade_au-p25_grade_au));
                let material_cu_grade = super::mesh_handlers::color_scale((cu-p25_grade_cu)/(p75_grade_cu-p25_grade_cu));

                let mut prisma_cu = prisma_mesh.clone();
                let mut prisma_au = prisma_mesh;

                prisma_au.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![material_au_grade; prisma_au.count_vertices()]);
                prisma_cu.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![material_cu_grade; prisma_cu.count_vertices()]);


                au_grades_meshes_result.push(prisma_au);
                cu_grades_meshes_result.push(prisma_cu);
                let transform = (grade_from_coord + grade_to_coord)*0.5;
                transforms_result.push(Transform::from_xyz(transform.x,transform.y,transform.z));

            }



        }

        let au_final_mesh = super::mesh_handlers::combine_meshes(au_grades_meshes_result,
                                                              transforms_result.clone(),
                                                              true, false,
                                                              false, true);

        let cu_final_mesh = super::mesh_handlers::combine_meshes(cu_grades_meshes_result,
                                                                 transforms_result,
                                                                 true, false,
                                                                 false, true);

        //TODO
        vec![au_final_mesh, cu_final_mesh]
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

