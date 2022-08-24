use bevy::prelude::*;

use crate::game::CleanupOnGameplayEnd;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct MenuCamera;

/// Initialise the camera.
pub fn create_camera(mut commands: Commands) {
    let bundle = Camera2dBundle::default();
    commands
        .spawn_bundle(bundle)
        .insert(GameCamera)
        .insert(CleanupOnGameplayEnd);
}

/// This ensures that the camera scales when the window is resized.
pub fn set_cam_scale(windows: Res<Windows>, mut query: Query<&mut Transform, With<Camera>>) {
    let window = get_primary_window_size(&windows);
    for mut transform in query.iter_mut() {
        let multiplier = 4.;
        let width = 16. * multiplier;
        let height = 9. * multiplier;
        let scale = (height / window.y).max(width / window.x);
        transform.scale = Vec3::new(scale, scale, 1.);
        transform.translation.x = width * 0.5;
        transform.translation.y = height * 0.5;
        trace!("Scale={:?} || Size={:?}", scale, window);
    }
}

pub fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}
