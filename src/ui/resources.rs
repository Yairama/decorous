use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct UIState {
    pub label: String,
    pub value: f32,
    pub inverted: bool,
    pub is_window_open: bool,
}

