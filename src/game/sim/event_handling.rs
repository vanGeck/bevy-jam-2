use crate::config::data_items::ItemsData;
use bevy::prelude::*;

use crate::game::{find_free_space, Item, ItemId, SpawnItemEvent};
use crate::positioning::{Coords, GridData};

/// Handle a looting session.
pub struct SimLootEvent(pub ItemId);

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
            let free_coords = find_free_space(&grid, dimens, &items_query);
            if same_tick_items.contains(&free_coords.unwrap()) {
                let new_free_coords = find_free_space(&grid, dimens, &items_query);
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
