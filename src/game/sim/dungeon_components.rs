use std::fmt::Formatter;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::combat::{DropTable, Enemy};

#[derive(Debug)]
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

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "init: {}, corr: {}, door: {}, desc: {}, srch: {}, psrch: {}, end: {}, start: {}, cmbt: {}",
               self.init, self.corridor, self.door, self.description, self.search, self.post_search, self.end, self.start, self.combat)
    }
}

impl Room {
    // Helper method for listing reults of dungeon generation.
    pub fn print_diag_name(&self) {
        if self.corridor {
            debug!("{}", "|Corridor|".to_string());
        } else if self.start {
            debug!("{}", "|First|".to_string());
        } else if self.end {
            debug!("{}", "|Last|".to_string());
        } else if self.combat {
            debug!("{}", "|Fight|".to_string());
        } else {
            debug!("{}", "|Empty|".to_string());
        }
    }
}

pub struct DungeonLevel {
    pub depth: i32,
    pub rooms: Vec<Room>,
    pub enemies: Vec<Enemy>,
    pub loot: DropTable,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TextType {
    RoomStart,
    RoomEnd,
    EnteredRoom,
    Corridor,
    Door,
    SearchingRoom,
    SearchingBody,
    FoundLoot,
    FoundNothing,
    CombatEnemyHit,
    CombatHeroHit,
    CombatNoResolution,
    CombatEnemyDied,
    CombatHeroDied,
    EnteredStartRoom,
    EnteredEndRoom,
    EnterRat,
    EnterGoblinBrat,
    EnterGoblinSwordsman,
    EnterGoblinShieldBearer,
}
