use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config;
use crate::config::config_items::ItemsConfig;
use crate::game::CleanupOnGameplayEnd;
use crate::positioning::Coords;
use crate::positioning::Depth;
use crate::positioning::Dimens;
use crate::positioning::Pos;

use super::components::Item;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub sprite_path: String,
    pub width: i32,
    pub height: i32,
}

pub struct ItemSpawnTimer(Timer);

pub fn setup_spawn_item_timer(mut commands: Commands) {
    commands.insert_resource(ItemSpawnTimer(Timer::from_seconds(5.0, true))); // Ref 1
}

pub fn spawn_item_timer_system(
    time: Res<Time>,
    mut timer: ResMut<ItemSpawnTimer>,
    items: Res<config::config_items::ItemsConfig>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    // update our timer with the time elapsed since the last update
    if timer.0.tick(time.delta()).just_finished() {
        let item_data = items.get_random_item();
        let item = Item {
            id: item_data.id.clone(),
            name: item_data.name.clone(),
            sprite_path: item_data.sprite_path.clone(),
            width: item_data.width,
            height: item_data.height,
        };
        let coords = Coords {
            pos: Pos::new(0, 0),
            dimens: Dimens::new(item.width, item.height),
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
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        debug!("Received spawn item event: {:?}", event);
        match event {
            SpawnItemEvent { item, coords } => {
                let sprite_path = std::path::PathBuf::new()
                    .join("textures/")
                    .join(&item.sprite_path);

                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(coords.dimens.as_vec2()),
                            ..default()
                        },
                        texture: asset_server.load(sprite_path),
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
