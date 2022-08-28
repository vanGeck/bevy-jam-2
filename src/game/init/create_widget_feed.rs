use bevy::prelude::*;

use crate::config::data_layout::LayoutData;
use crate::game::feed::{EventFeed, EventFeedContainer};
use crate::game::CleanupOnGameplayEnd;
use crate::positioning::Depth;

pub fn create_layout_feed(mut commands: Commands, layout: Res<LayoutData>) {
    let x = layout.left_x();
    let width = layout.left_width();
    let y = layout.c_left.feed_y();
    let height = layout.c_left.feed_height(&layout);
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
        .insert(Name::new("EventFeed"))
        .insert(EventFeedContainer)
        .insert(CleanupOnGameplayEnd);
    commands.insert_resource(EventFeed { next_id: 0 })
}
