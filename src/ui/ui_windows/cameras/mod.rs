pub mod camera_3d_free;
pub mod camera_3d_panorbit;
use super::scenes::NotInScene;

use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use bevy::utils::HashSet;
use bevy::window::WindowRef;
use bevy::{prelude::*, render::primitives::Aabb};
use bevy_infinite_grid::GridShadowCamera;
use crate::ui::ui_core::{
    editor_window::{EditorWindow, EditorWindowContext},
    Editor, EditorEvent,
};
use bevy_egui::egui;
use bevy_mod_picking::prelude::RaycastPickCamera;
// use bevy_mod_picking::prelude::PickRaycastSource;

use super::hierarchy::{HideInEditor, HierarchyWindow};

use self::camera_3d_panorbit::PanOrbitCamera;


pub const EDITOR_RENDER_LAYER: u8 = 19;

// Present on all editor cameras
#[derive(Component)]
pub struct EditorCamera;

// Present only one the one currently active camera
#[derive(Component)]
pub struct ActiveEditorCamera;

// Marker component for the 3d free camera
#[derive(Component)]
struct EditorCamera3dFree;

// Marker component for the 3d pan+orbit
#[derive(Component)]
struct EditorCamera3dPanOrbit;

pub struct CameraWindow;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum EditorCamKind {
    D3Free,
    #[default]
    D3PanOrbit,
}

impl EditorCamKind {
    fn name(self) -> &'static str {
        match self {
            EditorCamKind::D3Free => "3D (Free)",
            EditorCamKind::D3PanOrbit => "3D (Pan/Orbit)",
        }
    }

    fn all() -> [EditorCamKind; 2] {
        [
            EditorCamKind::D3Free,
            EditorCamKind::D3PanOrbit,
        ]
    }
}



#[derive(Default)]
pub struct CameraWindowState {
    // make sure to keep the `ActiveEditorCamera` marker component in sync with this field
    editor_cam: EditorCamKind,
    pub show_ui: bool,
}

impl CameraWindowState {
    pub fn editor_cam(&self) -> EditorCamKind {
        self.editor_cam
    }
}

impl EditorWindow for CameraWindow {
    type State = CameraWindowState;

    const NAME: &'static str = "Cameras";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        cameras_ui(ui, world);
    }

    fn viewport_toolbar_ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<CameraWindow>().unwrap();
        ui.menu_button(state.editor_cam.name(), |ui| {
            for camera in EditorCamKind::all() {
                ui.horizontal(|ui| {
                    if ui.button(camera.name()).clicked() {
                        if state.editor_cam != camera {
                            set_active_editor_camera_marker(world, camera);
                        }

                        state.editor_cam = camera;

                        ui.close_menu();
                    }
                });
            }
        });
        ui.checkbox(&mut state.show_ui, "UI");
    }

    fn app_setup(app: &mut App) {
        app.init_resource::<PreviouslyActiveCameras>();

        app.add_plugin(camera_3d_free::FlycamPlugin)
            .add_plugin(camera_3d_panorbit::PanOrbitCameraPlugin)
            .add_system(
                set_editor_cam_active
                    .before(camera_3d_panorbit::CameraSystem::Movement)
                    .before(camera_3d_free::CameraSystem::Movement),
            )
            .add_system(toggle_editor_cam.in_base_set(CoreSet::PreUpdate))
            .add_system(focus_selected.in_base_set(CoreSet::PreUpdate))
            .add_system(initial_camera_setup);
        app.add_startup_system(spawn_editor_cameras.in_base_set(StartupSet::PreStartup));

        app.add_system(
            set_main_pass_viewport
                .in_base_set(CoreSet::PostUpdate)
                .after(crate::ui::ui_core::EditorSet::UI)
                .before(bevy::render::camera::CameraUpdateSystem),
        );
    }
}

fn set_active_editor_camera_marker(world: &mut World, editor_cam: EditorCamKind) {
    let mut previously_active = world.query_filtered::<Entity, With<ActiveEditorCamera>>();
    let mut previously_active_iter = previously_active.iter(world);
    let previously_active = previously_active_iter.next();

    assert!(
        previously_active_iter.next().is_none(),
        "there should be only one `ActiveEditorCamera`"
    );

    if let Some(previously_active) = previously_active {
        world
            .entity_mut(previously_active)
            .remove::<ActiveEditorCamera>();
    }

    let entity = match editor_cam {
        EditorCamKind::D3Free => {
            let mut state = world.query_filtered::<Entity, With<EditorCamera3dFree>>();
            state.iter(world).next().unwrap()
        }
        EditorCamKind::D3PanOrbit => {
            let mut state = world.query_filtered::<Entity, With<EditorCamera3dPanOrbit>>();
            state.iter(world).next().unwrap()
        }
    };
    world.entity_mut(entity).insert(ActiveEditorCamera);
}

fn cameras_ui(ui: &mut egui::Ui, world: &mut World) {
    // let cameras = active_cameras.all_sorted();
    // let mut query: QueryState<&Camera> = world.query();
    // for camera in query.iter(world) {
    //
    // }

    let prev_cams = world.resource::<PreviouslyActiveCameras>();

    ui.label("Cameras");
    for cam in prev_cams.0.iter() {
        ui.horizontal(|ui| {
            // let active = curr_active.or(prev_active);
            //
            // let text = egui::RichText::new("👁").heading();
            // let show_hide_button = egui::Button::new(text).frame(false);
            // if ui.add(show_hide_button).clicked() {
            //     toggle_cam_visibility = Some((camera.to_string(), active));
            // }
            //
            // if active.is_none() {
            //     ui.set_enabled(false);
            // }

            ui.label(format!("{}: {:?}", "Camera", cam));
        });
    }
}

fn spawn_editor_cameras(mut commands: Commands, editor: Res<Editor>) {
    #[derive(Component, Default)]
    struct Ec2d;
    #[derive(Component, Default)]
    struct Ec3d;

    info!("Spawning editor cameras");

    let render_layers = RenderLayers::default().with(EDITOR_RENDER_LAYER);

    let show_ui_by_default = false;
    let editor_cam_priority = 100;

    let target = RenderTarget::Window(WindowRef::Entity(editor.window()));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: editor_cam_priority,
                is_active: false,
                target: target.clone(),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 5.0),
            ..Camera3dBundle::default()
        },
        UiCameraConfig {
            show_ui: show_ui_by_default,
        },
        Ec3d,
        camera_3d_free::FlycamControls::default(),
        EditorCamera,
        EditorCamera3dFree,
        HideInEditor,
        Name::new("Editor Camera 3D Free"),
        NotInScene,
        render_layers,
        RaycastPickCamera::default(),
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                //  Prevent multiple cameras from having the same priority.
                order: editor_cam_priority + 1,
                target: target.clone(),
                is_active: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 5.0),
            ..Camera3dBundle::default()
        },
        UiCameraConfig {
            show_ui: show_ui_by_default,
        },
        Ec3d,
        PanOrbitCamera::default(),
        EditorCamera,
        EditorCamera3dPanOrbit,
        HideInEditor,
        Name::new("Editor Camera 3D Pan/Orbit"),
        NotInScene,
        render_layers,
        RaycastPickCamera::default(),
        ActiveEditorCamera
    )).insert(GridShadowCamera);

}

fn set_editor_cam_active(
    editor: Res<Editor>,

    mut editor_cameras: ParamSet<(
        Query<(&mut Camera, &mut camera_3d_free::FlycamControls)>,
        Query<(&mut Camera, &mut camera_3d_panorbit::PanOrbitCamera)>
    )>,

    mut ui_camera_settings: Query<&mut UiCameraConfig, With<EditorCamera>>,
) {
    let camera_window_state = &editor.window_state::<CameraWindow>().unwrap();
    let editor_cam = camera_window_state.editor_cam;

    if editor.active() {
        ui_camera_settings
            .for_each_mut(|mut settings| settings.show_ui = camera_window_state.show_ui);
    }

    {
        let mut q = editor_cameras.p0();
        let mut editor_cam_3d_free = q.single_mut();
        let active = matches!(editor_cam, EditorCamKind::D3Free) && editor.active();
        editor_cam_3d_free.0.is_active = active;
        editor_cam_3d_free.1.enable_movement = active && !editor.listening_for_text();
        editor_cam_3d_free.1.enable_look = active && editor.viewport_interaction_active();
    }
    {
        let mut q = editor_cameras.p1();
        let mut editor_cam_3d_panorbit = q.single_mut();
        let active = matches!(editor_cam, EditorCamKind::D3PanOrbit) && editor.active();
        editor_cam_3d_panorbit.0.is_active = active;
        editor_cam_3d_panorbit.1.enabled = active && editor.viewport_interaction_active();
    }

}

#[derive(Resource, Default)]
struct PreviouslyActiveCameras(HashSet<Entity>);

fn toggle_editor_cam(
    editor: Res<Editor>,
    mut editor_events: EventReader<EditorEvent>,
    mut prev_active_cams: ResMut<PreviouslyActiveCameras>,
    mut cam_query: Query<(Entity, &mut Camera)>,
) {
    if editor.always_active() {
        return;
    }

    for event in editor_events.iter() {
        if let EditorEvent::Toggle { now_active } = *event {
            if now_active {
                // Add all currently active cameras
                for (e, mut cam) in cam_query
                    .iter_mut()
                    //  Ignore non-Window render targets
                    .filter(|(_e, c)| matches!(c.target, RenderTarget::Window(_)))
                    .filter(|(_e, c)| c.is_active)
                {
                    prev_active_cams.0.insert(e);
                    cam.is_active = false;
                }
            } else {
                for cam in prev_active_cams.0.iter() {
                    if let Ok((_e, mut camera)) = cam_query.get_mut(*cam) {
                        camera.is_active = true;
                    }
                }
                prev_active_cams.0.clear();
            }
        }
    }
}

fn focus_selected(
    mut editor_events: EventReader<EditorEvent>,
    mut active_cam: Query<
        (
            &mut Transform,
            Option<&mut PanOrbitCamera>,
            Option<&mut OrthographicProjection>,
        ),
        With<ActiveEditorCamera>,
    >,
    selected_query: Query<
        (&GlobalTransform, Option<&Aabb>, Option<&Sprite>),
        Without<ActiveEditorCamera>,
    >,
    editor: Res<Editor>,
    window: Query<&Window>,
) {
    let Ok(window) = window.get(editor.window()) else { return };

    for event in editor_events.iter() {
        match *event {
            EditorEvent::FocusSelected => (),
            _ => continue,
        }

        let hierarchy = editor.window_state::<HierarchyWindow>().unwrap();
        if hierarchy.selected.is_empty() {
            info!("Coudldn't focus on selection because selection is empty");
            return;
        }

        let (bounds_min, bounds_max) = hierarchy
            .selected
            .iter()
            .filter_map(|selected_e| {
                selected_query
                    .get(selected_e)
                    .map(|(&tf, bounds, sprite)| {
                        let default_value = (tf.translation(), tf.translation());
                        let sprite_size = sprite
                            .map(|s| s.custom_size.unwrap_or(Vec2::ONE))
                            .map_or(default_value, |sprite_size| {
                                (
                                    tf * Vec3::from((sprite_size * -0.5, 0.0)),
                                    tf * Vec3::from((sprite_size * 0.5, 0.0)),
                                )
                            });

                        bounds.map_or(sprite_size, |bounds| {
                            (tf * Vec3::from(bounds.min()), tf * Vec3::from(bounds.max()))
                        })
                    })
                    .ok()
            })
            .fold(
                (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN)),
                |(acc_min, acc_max), (min, max)| (acc_min.min(min), acc_max.max(max)),
            );

        const RADIUS_MULTIPLIER: f32 = 2.0;

        let bounds_size = bounds_max - bounds_min;
        let focus_loc = bounds_min + bounds_size * 0.5;
        let radius = if bounds_size.max_element() > f32::EPSILON {
            bounds_size.length() * RADIUS_MULTIPLIER
        } else {
            RADIUS_MULTIPLIER
        };

        let (mut camera_tf, pan_orbit_cam, ortho) = active_cam.single_mut();

        if let Some(mut ortho) = ortho {
            camera_tf.translation.x = focus_loc.x;
            camera_tf.translation.y = focus_loc.y;

            ortho.scale = radius / window.width().min(window.height()).max(1.0);
        } else {
            camera_tf.translation = focus_loc + camera_tf.rotation.mul_vec3(Vec3::Z) * radius;
        }

        if let Some(mut pan_orbit_cam) = pan_orbit_cam {
            pan_orbit_cam.focus = focus_loc;
            pan_orbit_cam.radius = radius;
        }

        let len = hierarchy.selected.len();
        let noun = if len == 1 { "entity" } else { "entities" };
        info!("Focused on {} {}", len, noun);
    }
}

fn initial_camera_setup(
    mut has_decided_initial_cam: Local<bool>,
    mut was_positioned_3d: Local<bool>,

    mut commands: Commands,
    mut editor: ResMut<Editor>,

    mut cameras: ParamSet<(
        // 3d free
        Query<
            (Entity, &mut Transform, &mut camera_3d_free::FlycamControls),
            With<EditorCamera3dFree>,
        >,
        // 3d pan/orbit
        Query<
            (
                Entity,
                &mut Transform,
                &mut PanOrbitCamera,
            ),
            With<EditorCamera3dPanOrbit>,
        >,
        Query<&Transform, (With<Camera3d>, Without<EditorCamera>)>,
    )>,
) {
    let cam3d = cameras.p2().get_single().ok().cloned();

    if !*has_decided_initial_cam {
        let camera_state = editor.window_state_mut::<CameraWindow>().unwrap();

        match cam3d.is_some() {
            true => {
                camera_state.editor_cam = EditorCamKind::D3PanOrbit;
                commands
                    .entity(cameras.p1().single().0)
                    .insert(ActiveEditorCamera);
                *has_decided_initial_cam = true;
            }

            false => return
        }
    }

    if !*was_positioned_3d {
        if let Some(cam3d_transform) = cam3d {
            if !cam3d_transform.rotation.is_finite()
                || !cam3d_transform.translation.is_finite()
                || !cam3d_transform.scale.is_finite()
            {
                return;
            };

            {
                let mut query = cameras.p0();
                let (_, mut cam_transform, mut cam) = query.single_mut();
                *cam_transform = cam3d_transform;
                let (yaw, pitch, _) = cam3d_transform.rotation.to_euler(EulerRot::YXZ);
                cam.yaw = yaw;
                cam.pitch = pitch;
            }

            {
                let mut query = cameras.p1();
                let (_, mut cam_transform, mut cam) = query.single_mut();
                cam.radius = cam3d_transform.translation.distance(cam.focus);
                *cam_transform = cam3d_transform;
            }

            *was_positioned_3d = true;
        }
    }
}

fn set_main_pass_viewport(
    egui_settings: Res<bevy_inspector_egui::bevy_egui::EguiSettings>,
    editor: Res<Editor>,
    window: Query<&Window>,
    mut cameras: Query<&mut Camera, With<EditorCamera>>,
) {
    if !editor.is_changed() {
        return;
    };

    let Ok(window) = window.get(editor.window()) else { return };

    let viewport = editor.active().then(|| {
        let scale_factor = window.scale_factor() * egui_settings.scale_factor;
        let viewport_pos = editor.viewport().left_top().to_vec2() * scale_factor as f32;
        let viewport_size = editor.viewport().size() * scale_factor as f32;

        if !viewport_size.is_finite() {
            warn!("editor viewport size is infinite");
        }

        bevy::render::camera::Viewport {
            physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
            physical_size: UVec2::new(
                (viewport_size.x as u32).max(1),
                (viewport_size.y as u32).max(1),
            ),
            depth: 0.0..1.0,
        }
    });

    cameras.iter_mut().for_each(|mut cam| {
        cam.viewport = viewport.clone();
    });
}
