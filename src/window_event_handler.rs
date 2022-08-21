use bevy::prelude::*;
use bevy::window::WindowMode;

/// Handle some general behaviour related to the window that should be executed in any State.
pub fn handle_window(mut keys: ResMut<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let primary = windows.primary_mut();
    // Toggle fullscreen:
    if keys.clear_just_pressed(KeyCode::F11) {
        primary.set_mode(if primary.mode() != WindowMode::Windowed {
            WindowMode::Windowed
        } else {
            WindowMode::BorderlessFullscreen
        });
    }
}
