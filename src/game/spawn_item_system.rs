use bevy::prelude::*;

use crate::config::data_items::ItemsData;
use crate::game::items::Item;
use crate::game::{AssetStorage, CleanupOnGameplayEnd};
use crate::positioning::Coords;
use crate::positioning::Depth;
use crate::positioning::Pos;

pub struct ItemSpawnTimer(Timer);

pub fn setup_spawn_item_timer(mut commands: Commands) {
    commands.insert_resource(ItemSpawnTimer(Timer::from_seconds(5.0, true))); // Ref 1
}

pub fn spawn_item_timer_system(
    time: Res<Time>,
    mut timer: ResMut<ItemSpawnTimer>,
    items: Res<ItemsData>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    // update our timer with the time elapsed since the last update
    if timer.0.tick(time.delta()).just_finished() {
        let (dimens, item) = items.get_random_item();
        let coords = Coords {
            pos: Pos::new(0, 0),
            dimens,
        };
        spawn.send(SpawnItemEvent::new(item, coords));
    }
}

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
                        texture: assets.texture(&item.texture_id),
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

// References
// 1. Timers in Bevy
// https://bevyengine.org/learn/book/getting-started/resources/
