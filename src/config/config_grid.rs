use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::game::dragging::BeingDragged;
use crate::game::items::{EquipmentSlot, Item};
use crate::positioning::{Coords, Dimens, Pos};

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct GridConfig {
    pub event_feed: Coords,
    pub record_player: Coords,
    /// An invisible grid above the inventory grid, this is where new items spawn in.
    pub drop_in: Coords,
    /// This is where items are stored.
    pub inventory: Coords,
    /// A small crafting window used for complex recipes (of more than two ingredients).
    pub lower_bar: Coords,
    pub crafting: Coords,
    pub equipped: EquipmentGrid,
    pub combine: Coords,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct EquipmentGrid {
    /// The absolute coordinates of the equipment grid. Coordinates of each of the individual slots
    /// are relative to this.
    pub coords: Coords,
    pub slots: HashMap<EquipmentSlot, Coords>,
}

impl GridConfig {
    /// Loads the most relevant instance of `GridConfig`.
    ///
    /// If the `GridConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `GridConfig::default()`).
    ///
    /// If the 'GridConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> GridConfig {
        let override_file = get_config_override_dir().join("grid.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("grid.ron"))
        }
    }

    pub fn find_free_space(
        &self,
        dimens: Dimens,
        items_query: Query<&Coords, (With<Item>, Without<BeingDragged>)>, // is there any way to call this function without this query? it forces you to have the exact same query in whichever query you're calling this function from. - Jacques
    ) -> Option<Coords> {
        for y in 0..self.inventory.dimens.y {
            for x in 0..self.inventory.dimens.x {
                let coords = Coords {
                    pos: Pos::new(x, y),
                    dimens,
                };

                let overlap_conflict = items_query.iter().any(|item| coords.overlaps(item));
                let bound_conflict = !self.inventory.encloses(&coords);
                if !overlap_conflict && !bound_conflict {
                    return Some(coords);
                }
            }
        }
        None
    }
}

fn load_from_path(path: &Path) -> GridConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<GridConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the grid config file from {:?}! Falling back to GridConfig::default(). Error: {:?}",
                    path, error
                );
            GridConfig::default()
        })
}
