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
    for SimLootEvent(itemId) in events.iter() {
        trace!("Received sim loot event");
        if let Some((dimens, item)) = items_data.try_get_item(itemId.clone()) {
            let free_coords = find_free_space(&grid, dimens, &items_query);
            if let Some(coords) = free_coords {
                spawn.send(SpawnItemEvent::new(item, coords));
            }
        }
    }
}
