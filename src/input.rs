use bevy::prelude::*;

use crate::game::camera::GameCamera;

pub struct InputsPlugin;

#[derive(Component, Default)]
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
    windows: Res<Windows>,
    mut query_mouse: Query<(&mut Transform, &mut Mouse)>,
    query_cam: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok((camera, camera_transform)) = query_cam.get_single() {
        if let Ok((mut transform, mut mouse)) = query_mouse.get_single_mut() {
            // Bevy will not return anything here if the mouse is out of screen bounds...
            // ... unless a mouse button is pressed, for whatever reason.
            // That's why there's a double check for mouse being out of bounds.
            let window = windows.get_primary().unwrap();
            if let Some(screen_pos) = window.cursor_position() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE; // What the heck does ndc stand for?
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
                let world_position: Vec2 = world_position.truncate();

                mouse.position = world_position;
                mouse.screen_position = screen_pos;
                mouse.out_of_bounds = screen_pos.x < 0.
                    || screen_pos.x > window.width()
                    || screen_pos.y < 0.
                    || screen_pos.y > window.height();
                // The top-left corner is what the user clicks with,
                // so that should be the mouse position:
                transform.translation.x = mouse.position.x + 0.5;
                transform.translation.y = mouse.position.y - 0.5;
            } else {
                mouse.out_of_bounds = true;
            }
        }
    }
}
