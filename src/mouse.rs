use bevy::prelude::*;

use crate::game::camera::GameCamera;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, SpriteType};
use crate::positioning::depth::Depth;
use crate::positioning::dimens::Dimens;

#[derive(Component, Default)]
pub struct Mouse {
    /// Position in world coordinates.
    pub position: Vec2,
    /// Position in logical pixels in the window.
    pub screen_position: Vec2,
    /// Whether or not the mouse is currently in a dragging operation.
    ///
    /// It is handy to store this separately, rather than relying on whether or not the LMB is
    /// held down, because this way we could add an accessibility mode that
    /// allows click-to-start-dragging, click-to-stop-dragging.
    pub is_dragging: bool,
    pub out_of_bounds: bool,
}

pub fn configure_cursor(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    assets: Res<AssetStorage>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Dimens::new(2, 2).as_vec2()),
                index: 0,
                ..default()
            },
            texture_atlas: assets.atlas(&SpriteType::Cursor),
            transform: Transform::from_xyz(0., 0., Depth::Cursor.z()),
            ..Default::default()
        })
        .insert(Name::new("MouseCursor"))
        .insert(Mouse::default())
        .insert(CleanupOnGameplayEnd);
}

pub fn reset_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(true);
}

// TODO: Put this earlier in system execution order, to avoid 1 extra frame of delay for cursor.
pub fn calc_mouse_pos(
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

                transform.translation.x = mouse.position.x;
                transform.translation.y = mouse.position.y;

                mouse.position = world_position;
                mouse.screen_position = screen_pos;
                mouse.out_of_bounds = screen_pos.x < 0.
                    || screen_pos.x > window.width()
                    || screen_pos.y < 0.
                    || screen_pos.y > window.height();
            } else {
                mouse.out_of_bounds = true;
            }
        }
    }
}

pub fn set_cursor_sprite(mut query: Query<(&mut TextureAtlasSprite, &Mouse)>) {
    let (mut sprite, mouse) = query.single_mut();
    sprite.index = if mouse.is_dragging { 1 } else { 0 };
}
