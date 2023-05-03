
use std::f64::consts::PI;

pub fn interpolate_point_on_the_line(
    origin: [f32;3],
    azimuth: f32,
    dip: f32,
    distance: f32,
) -> [f32;3] {

    let azimuth_rad = azimuth * PI as f32/ 180.0;
    let dip_rad = dip * PI as f32/ 180.0;

    let vector_x = dip_rad.sin() * azimuth_rad.cos();
    let vector_y = dip_rad.sin() * azimuth_rad.sin();
    let vector_z = dip_rad.cos();

    let x = origin[0] + distance * vector_x;
    let y = origin[1] + distance * vector_y;
    let z = origin[2] + distance * vector_z;

    [x, y, z]
}
