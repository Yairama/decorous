use bevy::{prelude::*, pbr::{CascadeShadowConfigBuilder}};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle};
use bevy_mod_picking::prelude::RaycastPickCamera;
use crate::ui::ui_windows::hierarchy::HideInEditor;

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    }, Name::new("Grid"), HideInEditor));

    // commands
    //     .spawn((Camera3dBundle {
    //         camera: Camera {
    //             order: 150,
    //             is_active: false,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0.0, 4.37, 14.77),
    //         ..Default::default()
    //     },
    //         HideInEditor
    //     ));


    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {shadows_enabled:true,
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10000.0,
            ..default()
        }.into(),
        ..Default::default()
    });

    let mat = standard_materials.add(StandardMaterial::default());

    let cube = commands.spawn((PbrBundle {
        material: mat.clone(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    }, Name::new("Test Cube"))).id();

    let child_1 = commands.spawn(
        PbrBundle {
            material: mat.clone(),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: Transform::from_xyz(-5.0, 2.0, 0.0),
            ..default()
        }
    ).id();

    let child_2 = commands.spawn(
        PbrBundle {
            material: mat.clone(),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: Transform::from_xyz(5.0, 2.0, 0.0),
            ..default()
        }
    ).id();

    commands.entity(cube).push_children(
        &[child_1, child_2]
    );

}