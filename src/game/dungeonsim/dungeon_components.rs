use bevy::log::info;
use serde::{Deserialize, Serialize};

use crate::game::dungeonsim::combat::Combatant;

pub struct Room {
    // Flags used in room processing. Determine message ordering and room types.
    pub init: bool,
    pub corridor: bool,
    pub door: bool,
    pub description: bool,
    pub search: bool,
    pub post_search: bool,
    pub end: bool,
    pub start: bool,
    pub combat: bool,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            init: true,
            corridor: false,
            door: false,
            description: false,
            search: false,
            post_search: false,
            end: false,
            start: false,
            combat: false,
        }
    }
}

impl Room {
    // Helper method for listing reults of dungeon generation.
    pub fn print_diag_name(&self) {
        if self.corridor {
            info!("{}", "|Corridor|".to_string());
        } else if self.start {
            info!("{}", "|First|".to_string());
        } else if self.end {
            info!("{}", "|Last|".to_string());
        } else if self.combat {
            info!("{}", "|Fight|".to_string());
        } else {
            info!("{}", "|Empty|".to_string());
        }
    }
}

pub struct DungeonLevel {
    pub depth: i32,
    pub rooms: Vec<Room>,
    pub enemies: Vec<Combatant>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum TextType {
    RoomStart,
    RoomEnd,
    EnteredRoom,
    Corridor,
    Door,
    SearchingRoom,
    FoundLoot,
    FoundNothing,
    EnemyEncounter,
    CombatEnemyHit,
    CombatHeroHit,
    CombatNoResolution,
    CombatEnemyDied,
    CombatHeroDied,
    EnteredStartRoom,
    EnteredEndRoom,
}
