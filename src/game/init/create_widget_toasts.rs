use bevy::prelude::*;

use crate::config::data_layout::LayoutData;
use crate::game::CleanupOnGameplayEnd;
use crate::positioning::Depth;

pub fn create_layout_toasts(mut commands: Commands, layout: Res<LayoutData>) {
    let x = layout.middle_x();
    let width = layout.middle_width();
    let y = layout.c_mid.toasts.margin_bottom.unwrap_or(0.);
    let height = layout.c_mid.toasts.height.unwrap();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("Toasts"))
        .insert(CleanupOnGameplayEnd);
}
