use bevy::prelude::*;

use crate::game::{AssetStorage, CleanupOnGameplayEnd, SpriteType};
use crate::positioning::coords::Coords;
use crate::positioning::depth::Depth;

use super::components::Item;

/// Broadcast this as an event to spawn an item.
#[derive(Debug)]
pub struct SpawnItemEvent {
    item: Item,
    coords: Coords,
}

impl SpawnItemEvent {
    pub fn new(item: Item, coords: Coords) -> Self {
        SpawnItemEvent { item, coords }
    }
}

pub fn spawn_item(
    mut commands: Commands,
    mut events: EventReader<SpawnItemEvent>,
    assets: Res<AssetStorage>,
) {
    for event in events.iter() {
        debug!("Received spawn item event: {:?}", event);
        match event {
            SpawnItemEvent { item, coords } => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(coords.dimens.as_vec2()),
                            ..default()
                        },
                        texture: assets.texture(&SpriteType::Croissant),
                        transform: Transform::from_xyz(
                            coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                            coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                            Depth::Item.z(),
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new(item.name.clone()))
                    .insert(item.clone())
                    .insert(*coords)
                    .insert(CleanupOnGameplayEnd);
            }
        }
    }
}
