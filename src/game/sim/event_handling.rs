use crate::config::data_items::ItemsData;
use bevy::prelude::*;
use rand::Rng;
use crate::config::config_grid::GridConfig;

use crate::config::data_sim_texts::DungeonTexts;
use crate::game::sim::dungeon_components::TextType;
use crate::game::{BeingDragged, Item, SpawnItemEvent};
use crate::positioning::Coords;

/// Cause a message to be printed and maybe a sound to be played.
pub struct SimMessageEvent(pub TextType);
/// Handle a looting session.
pub struct SimLootEvent;

pub fn handle_sim_message(mut events: EventReader<SimMessageEvent>, texts: Res<DungeonTexts>) {
    for SimMessageEvent(text_type) in events.iter() {
        trace!("Received sim message event for TextType::{:?}", text_type);
        let random = pick_random_from_series(&texts.map.get(text_type).unwrap_or(&Vec::new()));
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
    grid: Res<GridConfig>,
    items_data: Res<ItemsData>,
    items_query: Query<&Coords, (With<Item>, Without<BeingDragged>)>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    for SimLootEvent in events.iter() {
        trace!("Received sim loot event");
        let (dimens, item) = items_data.get_random_item();
        let free_coords = grid.find_free_space(dimens, &items_query);
        if let Some(coords) = free_coords {
            spawn.send(SpawnItemEvent::new(item, coords));
        }
    }
}
