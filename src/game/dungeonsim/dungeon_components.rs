use bevy::log::debug;
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
            debug!("{}","|Corridor|".to_string());
        } else if self.start {
            debug!("{}","|First|".to_string());
        } else if self.end {
            debug!("{}","|Last|".to_string());
        } else if let Some(comb) = &self.monster {
            debug!("{}","|Fight|".to_string());
        } else {
            debug!("{}","|Empty|".to_string());
        }
    }
}

pub struct DungeonLevel {
    pub depth: i32,
    pub rooms: Vec<Room>
}