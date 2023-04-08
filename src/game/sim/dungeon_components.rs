use std::fmt::Formatter;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::combat::{DropTable, Enemy};
use crate::game::feed::MessageColour;

#[derive(Debug, Clone)]
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
    pub flavour: Option<TextType>,
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
            flavour: None,
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

#[derive(Clone)]
pub struct TimePoint {
    pub timepoint: i32,
    pub flavour: Option<TextType>,
}

impl Default for TimePoint {
    fn default() -> Self {
        TimePoint {
            timepoint: 0,
            flavour: None,
        }
    }
}

impl std::fmt::Display for TimePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "timepoint: {}", self.timepoint)
    }
}

#[derive(Clone)]
pub struct DungeonLevel {
    pub depth: i32,
    pub rooms: Vec<Room>,
    pub enemies: Vec<Enemy>,
    pub loot: Vec<DropTable>,
}

#[derive(Clone)]
pub struct TimePointLevel {
    pub timenum: i32,
    pub timepoints: Vec<TimePoint>,
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
    // enemy ecounters
    EnterRat,
    EnterGoblinBrat,
    EnterGoblinSwordsman,
    EnterGoblinShieldBearer,
    EnterOrcWarrior,
    EnterSkeleton,
    EnterZombie,
    EnterOgreNecromancer,
    // special room flavours
    PlantRoom,
    AlchemyLab,
    Armory,
    UndeadEntrance,
    LairEntrance,
}

impl TextType {
    pub fn colour_hint(&self) -> MessageColour {
        match self {
            TextType::EnterRat
            | TextType::EnterGoblinBrat
            | TextType::EnterGoblinSwordsman
            | TextType::EnterGoblinShieldBearer
            | TextType::EnterSkeleton
            | TextType::EnterZombie => MessageColour::MinorNegative,
            TextType::CombatHeroHit => MessageColour::MajorNegative,
            TextType::CombatEnemyHit => MessageColour::MinorPositive,
            TextType::CombatEnemyDied => MessageColour::MajorPositive,
            TextType::FoundLoot => MessageColour::MinorPositive,
            _ => MessageColour::Neutral,
        }
    }
}
