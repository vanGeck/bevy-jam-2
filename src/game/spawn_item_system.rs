use std::collections::HashMap;
use std::fs::File;
use bevy::prelude::*;
use rand::Rng;
use crate::game::{Item, SpawnItemEvent};
use crate::positioning::coordinates::Coordinates;
use crate::positioning::dimensions::Dimensions;
use crate::positioning::position::Position;
use serde::{Deserialize, Serialize};


pub struct Items {
    pub AllItemData: HashMap<String, ItemData>,
}

impl Items {
    pub fn get_random_item(&self) -> &ItemData {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.AllItemData.len());
        let mut item_keys: Vec<&String> = self.AllItemData.keys().collect();
        item_keys.swap(0, index);
        let item_key = item_keys[0];
        self.AllItemData.get(item_key).unwrap().clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub sprite_path: String,
    pub width: i32,
    pub height: i32,
}

struct ItemSpawnTimer(Timer);

pub fn setup_item_spawn_system(commands: &mut Commands) {
    let mut items: Items = Items { AllItemData: Default::default() };

    // read items.ron file
    let mut items_file = File::open("assets/items.ron").unwrap();

    // parse all items from items_file as ItemData
    let items_data: Vec<ItemData> = ron::de::from_reader(&mut items_file).unwrap();

    // insert all item data into items.AllItemData
    let mut items_map: HashMap<String, ItemData> = HashMap::new();
    for item_data in items_data {
        items_map.insert(item_data.id.clone(), item_data);
    }

    items.AllItemData = items_map;

    commands.insert_resource(items);
    commands.insert_resource(ItemSpawnTimer(Timer::from_seconds(15.0, true)));
}

pub fn spawn_item_system(time: Res<Time>, mut timer: Res<ItemSpawnTimer>, items: Res<Items>, mut spawn: EventWriter<SpawnItemEvent>) {
    // update our timer with the time elapsed since the last update
    if timer.0.tick(time.delta()).just_finished() {
        let itemData = items.get_random_item();
        let item = Item {
            id: itemData.id.clone(),
            name: itemData.name.clone(),
            sprite_path: itemData.sprite_path.clone(),
            width: itemData.width,
            height: itemData.height,
        };
        let coords = Coordinates {
            position: Position::new(0, 0),
            dimensions: Dimensions::new(item.width, item.height),
        };
        spawn.send(SpawnItemEvent::new(item, coords));
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

