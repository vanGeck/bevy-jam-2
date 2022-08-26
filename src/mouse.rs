use bevy::prelude::*;

use crate::game::camera::GameCamera;

#[derive(Default)]
pub struct Mouse {
    /// Position in world coordinates.
    pub position: Vec2,
    /// Position in logical pixels in the window.
    pub screen_position: Vec2,
    /// Position in logical pixels in the window inverted (needed for UI)
    pub screen_pos_inverted: Vec2,
    /// Whether or not the mouse is currently in a dragging operation.
    ///
    /// It is handy to store this separately, rather than relying on whether or not the LMB is
    /// held down, because this way we could add an accessibility mode that
    /// allows click-to-start-dragging, click-to-stop-dragging.
    pub is_dragging: bool,
    pub can_drag: bool,
    pub out_of_bounds: bool,
}

pub fn reset_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_icon(CursorIcon::Default);
}

pub fn calc_mouse_pos(
    windows: Res<Windows>,
    mut mouse: ResMut<Mouse>,
    query_cam: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok((camera, camera_transform)) = query_cam.get_single() {
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
            mouse.screen_pos_inverted = Vec2::new(screen_pos.x, window.height() - screen_pos.y);
            mouse.out_of_bounds = screen_pos.x < 0.
                || screen_pos.x > window.width()
                || screen_pos.y < 0.
                || screen_pos.y > window.height();
        } else {
            mouse.out_of_bounds = true;
        }
    }
}

pub fn set_cursor_appearance(mut windows: ResMut<Windows>, mouse: Res<Mouse>) {
    if mouse.out_of_bounds {
        return;
    }
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_icon(if mouse.is_dragging {
        CursorIcon::Grabbing
    } else if mouse.can_drag {
        CursorIcon::Move
    } else {
        CursorIcon::Default
    });
}

// References
// 1. calc_mouse_pos
// https://bevy-cheatbook.github.io/cookbook/cursor2world.html
