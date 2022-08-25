use bevy::prelude::*;

use crate::config::data_layout::LayoutData;
use crate::game::CleanupOnGameplayEnd;

const MAGIC: f32 = 4.;
// 16x9 is the most common aspect ratio, make it look the best for those screens.
// (Black bars down the sides for non-standard screens)
const ASPECT_RATIO_X: f32 = 16.;
const ASPECT_RATIO_Y: f32 = 9.;

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
pub fn set_cam_scale(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<Camera>>,
    layout: Res<LayoutData>,
) {
    let window = get_primary_window_size(&windows);
    for mut transform in query.iter_mut() {
        let multiplier = MAGIC * layout.factor;
        let width = ASPECT_RATIO_X * multiplier;
        let height = ASPECT_RATIO_Y * multiplier;
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
