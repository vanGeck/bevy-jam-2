use bevy::log::{debug, info};
use rand::Rng;
use crate::config::dungeon_params::DungeonParams;
use crate::default;
use crate::game::dungeonsim::combat::Combatant;
use crate::game::dungeonsim::dungeon_components::{DungeonLevel, Room};

pub fn generate_level(len: i32, params: &DungeonParams) -> DungeonLevel {
    let mut rooms = Vec::<Room>::new();
    let mut rng = rand::thread_rng();
    
    rooms.push(generate_first_room());
    
    for i in 1..(len-1) {
        let x = rng.gen_range(0.0..1.0);
        if x < params.chance_corridor {
            rooms.push(generate_corridor())
        } else if x < params.chance_empty + params.chance_corridor {
            rooms.push(generate_empty())
        } else if x < params.chance_empty + params.chance_corridor + params.chance_fight {
            rooms.push(generate_fight())
        }
    }
    rooms.push(generate_last_room());

    info!("Dungeon generation results: ");
    for s in 0..rooms.len(){
        rooms[s].print_diag_name();
    }

    return DungeonLevel{
        depth: 0,
        rooms
    }
}

fn generate_first_room() -> Room {
    Room{
        start: true,
        ..Default::default()
    }
}

fn generate_last_room() -> Room {
    Room{
        end: true,
        ..Default::default()
    }
}

fn generate_corridor() -> Room {
    Room{
        corridor: true,
        ..Default::default()
    }
}

fn generate_empty() -> Room {
    Room{
        door: true,
        description: true,
        search: true,
        ..Default::default()
    }
}

fn generate_fight() -> Room {
    Room{
        door: true,
        description: true,
        search: true,
        monster: Option::from(Combatant {
            health: 10,
            proficiency: 0,
            damage_res: 0,
            damage_bonus: 0
        }),
        ..Default::default()
    }
}