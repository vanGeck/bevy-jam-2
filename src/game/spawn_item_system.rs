use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
use bevy::prelude::*;
use rand::Rng;
use crate::game::{Item, SpawnItemEvent};
use crate::config::config_items::ItemsConfig;
use crate::positioning::coordinates::Coordinates;
use crate::positioning::dimensions::Dimensions;
use crate::positioning::position::Position;
use serde::{Deserialize, Serialize};
use crate::config;


pub struct Items {
    pub all_items_data: HashMap<String, ItemData>,
}

impl Items {
    pub fn get_random_item(&self) -> &ItemData {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.all_items_data.len());
        let mut item_keys: Vec<&String> = self.all_items_data.keys().collect();
        item_keys.swap(0, index);
        let item_key = item_keys[0];
        self.all_items_data.get(item_key).unwrap().clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub sprite_path: String,
    pub width: i32,
    pub height: i32,
}

pub struct ItemSpawnTimer(Timer);


// pub fn setup_item_spawn_system(mut commands: Commands) {
//     let mut items: Items = Items { all_items_data: Default::default() };
//
//     // read items.ron file
//
//     let items_file = fs::read_to_string().unwrap();
//
//     // parse all items from items_file as ItemData
//     let items_data: Vec<ItemData> = ron::de::from_str(&items_file).unwrap();
//
//     // insert all item data into items.AllItemData
//     let mut items_map: HashMap<String, ItemData> = HashMap::new();
//     for item_data in items_data {
//         items_map.insert(item_data.id.clone(), item_data);
//     }
//
//     items.all_items_data = items_map;
//
//     commands.insert_resource(items);
// }

pub fn setup_spawn_item_timer(mut commands: Commands) {
    commands.insert_resource(ItemSpawnTimer(Timer::from_seconds(15.0, true)));
}

pub fn spawn_item_system(time: Res<Time>, mut timer: ResMut<ItemSpawnTimer>, items: Res<config::config_items::ItemsConfig>, mut spawn: EventWriter<SpawnItemEvent>) {
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
        let coordinates = Coordinates {
            position: Position::new(0, 0),
            dimensions: Dimensions::new(item.width, item.height),
        };
        spawn.send(SpawnItemEvent::new(item, coordinates));
    }


    // TODO: Remove these test spawns later:
    // spawn.send(SpawnItemEvent::new(
    //     Item {
    //         name: "Croissant".to_string(),
    //     },
    //     Coordinates::new(Position::new(1, 1), Dimensions::new(3, 2)),
    // ));
    // spawn.send(SpawnItemEvent::new(
    //     Item {
    //         name: "Croissant2".to_string(),
    //     },
    //     Coordinates::new(Position::new(10, 10), Dimensions::new(3, 2)),
    // ));
}

