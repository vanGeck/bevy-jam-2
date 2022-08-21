use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::camera::GameCamera;

pub struct InputsPlugin;

#[derive(Default, Inspectable)]
pub struct Mouse {
    /// Position in world coordinates.
    pub position: Vec2,
    /// Position in logical pixels in the window.
    pub screen_position: Vec2,
    /// TODO: is this really necessary?
    /// Whether or not the mouse is currently in a dragging operation.
    pub is_dragging: bool,
    pub out_of_bounds: bool,
}

pub fn world_cursor_system(
    mut mouse_position: ResMut<Mouse>,
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok((camera, camera_transform)) = query.get_single() {
        let window = windows.get_primary().unwrap();

        // Bevy will not return anything here if the mouse is out of screen bounds...
        // ... unless a mouse button is pressed, for whatever reason.
        // That's why there's a double check for mouse being out of bounds.
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE; // What the heck does ndc stand for?
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world_position: Vec2 = world_position.truncate();

            mouse_position.position = world_position;
            mouse_position.screen_position = screen_pos;
            mouse_position.out_of_bounds = screen_pos.x < 0.
                || screen_pos.x > window.width()
                || screen_pos.y < 0.
                || screen_pos.y > window.height();
        } else {
            mouse_position.out_of_bounds = true;
        }
    }
}
