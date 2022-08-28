use crate::config::data_items::ItemsData;
use bevy::prelude::*;
use rand::Rng;

use crate::config::data_sim_texts::DungeonTexts;
use crate::game::sim::dungeon_components::TextType;
use crate::game::{find_free_space, Item, ItemId, SpawnItemEvent};
use crate::positioning::{Coords, GridData};

/// Cause a message to be printed and maybe a sound to be played.
pub struct SimMessageEvent(pub TextType);
/// Handle a looting session.
pub struct SimLootEvent(pub ItemId);

pub fn handle_sim_message(mut events: EventReader<SimMessageEvent>, texts: Res<DungeonTexts>) {
    for SimMessageEvent(text_type) in events.iter() {
        trace!("Received sim message event for TextType::{:?}", text_type);
        let random = pick_random_from_series(texts.map.get(text_type).unwrap_or(&Vec::new()));
        if let Some(message) = random {
            println!("{}", message);
        } else {
            error!("Missing or empty dungeon text: TextType::{:?}", text_type);
        }
    }
}
fn pick_random_from_series(strings: &Vec<String>) -> Option<String> {
    if strings.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..strings.len()) as usize;
        strings.get(idx).cloned()
    }
}

pub fn handle_sim_loot(
    mut events: EventReader<SimLootEvent>,
    grid: Res<GridData>,
    items_data: Res<ItemsData>,
    items_query: Query<&Coords, With<Item>>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    let mut same_tick_items: Vec<Coords> = Vec::new();
    for SimLootEvent(item_id) in events.iter() {
        trace!("Received sim loot event");
        if let Some((dimens, item)) = items_data.try_get_item(item_id.clone()) {
            let free_coords = find_free_space(&grid, dimens, &items_query, &same_tick_items);
            if same_tick_items.contains(&free_coords.unwrap()) {
                let new_free_coords =
                    find_free_space(&grid, dimens, &items_query, &same_tick_items);
                if let Some(coords) = new_free_coords {
                    spawn.send(SpawnItemEvent::new(item, coords));
                }
            } else if let Some(coords) = free_coords {
                same_tick_items.push(coords);
                spawn.send(SpawnItemEvent::new(item, coords));
            }
        }
    }
}
