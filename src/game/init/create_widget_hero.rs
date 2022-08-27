use bevy::prelude::*;

use crate::config::data_layout::LayoutData;
use crate::game::{CleanupOnGameplayEnd, HealthBar};
use crate::positioning::Depth;

pub fn create_layout_hero(mut commands: Commands, layout: Res<LayoutData>) {
    let x = layout.factor * layout.right_x();
    let width = layout.factor * layout.right_width();
    let y = layout.factor * layout.c_right.hero_y();
    let height = layout.factor * layout.c_right.hero_height(&layout);
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
        .insert(Name::new("Hero"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            let health_bar_margin = 0.25;
            let health_bar_size = Vec2::new(width - health_bar_margin * 2., 0.25);
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(255., 0.2, 0.2, 0.8),
                        custom_size: Some(health_bar_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        0., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin,
                        11., // Relative to parent
                    ),
                    ..default()
                })
                .insert(Name::new("HealthBar"))
                .insert(HealthBar);
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.1, 0.1, 0.1, 1.),
                        custom_size: Some(health_bar_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        0.,
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin,
                        10., // Relative to parent
                    ),
                    ..default()
                })
                .insert(Name::new("HealthBarBackground"));
        });
}
