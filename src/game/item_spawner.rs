use bevy::prelude::*;

use crate::game::{AssetStorage, CleanupOnGameplayEnd, SpriteType};
use crate::positioning::coords::Coords;
use crate::positioning::depth::Depth;
use crate::positioning::dimens::Dimens;
use crate::positioning::pos::Pos;

use super::components::Item;

/// Broadcast this as an event to spawn an item.
#[derive(Debug)]
pub struct SpawnItemEvent {
    item: Item,
}

impl SpawnItemEvent {
    pub fn new(item: Item) -> Self {
        SpawnItemEvent { item }
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
            SpawnItemEvent { item } => {
                // TODO: make not hardcoded.
                let hardcoded = Coords::new(Pos::new(1, 1), Dimens::new(3, 2));
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(hardcoded.dimens.as_vec2()),
                            ..default()
                        },
                        texture: assets.texture(&SpriteType::Croissant),
                        transform: Transform::from_xyz(
                            hardcoded.pos.x as f32 + hardcoded.dimens.x as f32 * 0.5,
                            hardcoded.pos.y as f32 + hardcoded.dimens.y as f32 * 0.5,
                            Depth::Item.z(),
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new(item.name.clone()))
                    .insert(item.clone())
                    .insert(hardcoded)
                    .insert(CleanupOnGameplayEnd);
            }
        }
    }
}
