use bevy::log::{debug, info};
use crate::game::dungeonsim::combat::Combatant;

pub struct Room{
    // Flags used in room processing. Determine message ordering and room types.
    pub corridor: bool,
    pub door: bool,
    pub description: bool,
    pub search: bool,
    pub end: bool,
    pub start: bool,
    pub monster: Option<Combatant>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            corridor: false,
            door: false,
            description: false,
            search: false,
            end: false,
            start: false,
            monster: None,
        }
    }
}

impl Room {
    // Helper method for listing reults of dungeon generation.
    pub fn print_diag_name(&self) {
        if self.corridor {
            info!("{}","|Corridor|".to_string());
        } else if self.start {
            info!("{}","|First|".to_string());
        } else if self.end {
            info!("{}","|Last|".to_string());
        } else if let Some(comb) = &self.monster {
            info!("{}","|Fight|".to_string());
        } else {
            info!("{}","|Empty|".to_string());
        }
    }
}

pub struct DungeonLevel {
    pub depth: i32,
    pub rooms: Vec<Room>
}