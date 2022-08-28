use std::collections::HashMap;

use bevy::prelude::*;
use bevy::text::{Text2dBounds, Text2dSize};

use crate::config::data_layout::LayoutData;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, FontId};

/// Cause a message to be printed.
pub struct AddFeedItemEvent(pub String);

/// The event feed container.
#[derive(Component)]
pub struct EventFeedContainer;

/// An event feed item
#[derive(Component)]
pub struct EventFeedItem {
    pub id: i32,
}

#[derive(Component)]
pub struct EventFeedItemBg {
    pub id: i32,
}

/// Resource.
pub struct EventFeed {
    /// The next id that should be given out.
    pub next_id: i32,
}

impl EventFeed {
    pub fn next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub fn handle_add_to_feed(
    mut commands: Commands,
    assets: Res<AssetStorage>,
    mut events: EventReader<AddFeedItemEvent>,
    mut feed: ResMut<EventFeed>,
    layout: Res<LayoutData>,
    query_container: Query<Entity, With<EventFeedContainer>>,
) {
    for AddFeedItemEvent(message) in events.iter() {
        let text_style = TextStyle {
            font: assets.font(&FontId::FiraMonoMedium),
            font_size: 60.0,
            color: Color::rgba(1., 1., 1., 0.6),
        };
        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Left,
        };
        let container_width = layout.factor * layout.left_width();
        let container_height = layout.factor * layout.c_left.feed_height(&layout);
        let dimens_text = Vec2::new(
            container_width - 2. * layout.c_left.feed_padding,
            layout.c_left.feed_item_height,
        );
        let item_id = feed.next_id();
        let text = commands
            .spawn()
            .insert_bundle(Text2dBundle {
                // Default text, will probably never be seen:
                text: Text::from_section(message, text_style).with_alignment(text_alignment),
                // The max size that it should fit in:
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(
                        (dimens_text.x - layout.c_left.feed_padding) * layout.text_factor,
                        dimens_text.y * layout.text_factor,
                    ),
                },
                transform: Transform::from_translation(Vec3::new(
                    container_width * -0.5 + layout.c_left.feed_padding,
                    container_height * -0.5 + layout.c_left.feed_padding,
                    20.0, // Relative to the container.
                ))
                .with_scale(Vec3::new(
                    0.9 / layout.text_factor,
                    0.9 / layout.text_factor,
                    1.,
                )),
                ..default()
            })
            .insert(EventFeedItem { id: item_id })
            .insert(CleanupOnGameplayEnd)
            .id();
        let bg_color = if item_id.rem_euclid(2) == 0 {
            Color::rgba(0.1, 0.1, 0.1, 1.)
        } else {
            Color::rgba(0.15, 0.15, 0.15, 1.)
        };
        let text_background = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: bg_color,
                    custom_size: Some(Vec2::new(dimens_text.x, dimens_text.y)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    container_width * -0.5 + layout.c_left.feed_padding,
                    container_height * -0.5 + layout.c_left.feed_padding,
                    10., // Relative to the container.
                ),
                ..default()
            })
            .insert(EventFeedItemBg { id: item_id })
            .id();
        commands
            .entity(query_container.single())
            .push_children(&[text, text_background]);
    }
}

pub fn position_feed_item(
    mut commands: Commands,
    layout: Res<LayoutData>,
    mut queries: ParamSet<(
        Query<(Entity, &EventFeedItem, &mut Transform, &Text2dSize)>,
        Query<(Entity, &EventFeedItemBg, &mut Transform, &mut Sprite)>,
    )>,
) {
    let mut map: HashMap<i32, (f32, f32)> = HashMap::new();
    let highest_id = queries
        .p0()
        .iter()
        .fold(i32::MIN, |highest_id, (_, item, _, size)| {
            map.insert(item.id, (size.size.y, 0.));
            highest_id.max(item.id)
        });
    let mut running_total = 0.;
    for i in 0.. {
        let index = highest_id - i;
        let temp = map.get(&index);
        if temp.is_some() {
            running_total += temp.unwrap().0;
            map.insert(index, (temp.unwrap().0, running_total));
        } else {
            break;
        }
    }
    let container_width = layout.factor * layout.left_width();
    let container_height = layout.factor * layout.c_left.feed_height(&layout);
    let available_height = container_height - layout.c_left.feed_padding * 2.;
    // Set the text translation and delete those that are out of bounds.
    for (entity, item, mut transform, size) in queries.p0().iter_mut() {
        let (height, total_height) = map.get(&item.id).unwrap();
        if *total_height / layout.text_factor > available_height {
            trace!("Deleting text with id={:?} and size={:?}", item.id, size);
            commands.entity(entity).despawn_recursive();
        } else {
            transform.translation.x = container_width * -0.5 + layout.c_left.feed_padding * 2.;
            transform.translation.y = container_height * -0.5
                + layout.c_left.feed_padding
                + (total_height - height * 0.5) / layout.text_factor;
        }
    }
    // Set the background translation and delete those that are out of bounds.
    for (entity, item, mut transform, mut sprite) in queries.p1().iter_mut() {
        let (height, total_height) = map.get(&item.id).unwrap();
        if *total_height / layout.text_factor > available_height {
            trace!("Deleting bg with id={:?}", item.id);
            commands.entity(entity).despawn_recursive();
        } else {
            transform.translation.x = 0.;
            transform.translation.y = container_height * -0.5
                + (total_height - height * 0.5) / layout.text_factor
                + layout.c_left.feed_padding;
            sprite.custom_size = Some(Vec2::new(
                container_width - layout.c_left.feed_padding * 2.,
                map.get(&item.id).unwrap().0 / layout.text_factor,
            ));
        }
    }
}
