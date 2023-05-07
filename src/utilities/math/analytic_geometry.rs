

use bevy::math::Vec3;

pub fn interpolate_point_on_the_line(
    origin: [f32;3],
    azimuth: f32,
    dip: f32,
    distance: f32,
) -> Vec3 {

    let azimuth_rad = azimuth.to_radians();
    let dip_rad = dip.to_radians();

    let delta_x = distance * azimuth_rad.sin() * dip_rad.cos();
    let delta_y = distance * azimuth_rad.cos() * dip_rad.cos();
    let delta_z = distance * dip_rad.sin();

    let point_1 = [
        origin[0] + delta_x,
        origin[1] + delta_y,
        origin[2] + delta_z,
    ];

    let _point_2 = [
        origin[0] - delta_x,
        origin[1] - delta_y,
        origin[2] - delta_z,
    ];

    Vec3::new(point_1[0], point_1[2], point_1[1])
}
