use crate::config::config_sim::SimConfig;
use crate::game::sim::combat::Combatant;
use crate::game::sim::dungeon_components::{DungeonLevel, Room};
use bevy::prelude::*;
use rand::Rng;

pub fn generate_level(len: i32, params: &SimConfig, mut _cmd: &mut Commands) -> DungeonLevel {
    let mut rooms = Vec::<Room>::new();
    let mut fights = Vec::<Combatant>::new();
    let mut rng = rand::thread_rng();

    rooms.push(generate_first_room());
    fights.push(Combatant::default());

    for _ in 1..(len - 1) {
        let x = rng.gen_range(0.0..1.0);
        if x < params.chance_corridor {
            rooms.push(generate_corridor());
            fights.push(Combatant::default());
        } else if x < params.chance_empty + params.chance_corridor {
            rooms.push(generate_empty());
            fights.push(Combatant::default());
        } else if x < params.chance_empty + params.chance_corridor + params.chance_fight {
            rooms.push(generate_fight());
            fights.push(get_enemy());
        }
    }
    rooms.push(generate_last_room());
    fights.push(Combatant::default());

    info!("Dungeon generation results: ");
    for s in 0..rooms.len() {
        rooms[s].print_diag_name();
    }

    return DungeonLevel {
        depth: 0,
        rooms,
        enemies: fights,
    };
}

fn generate_first_room() -> Room {
    Room {
        start: true,
        ..Default::default()
    }
}

fn generate_last_room() -> Room {
    Room {
        end: true,
        ..Default::default()
    }
}

fn generate_corridor() -> Room {
    Room {
        corridor: true,
        ..Default::default()
    }
}

fn generate_empty() -> Room {
    Room {
        door: true,
        description: true,
        search: true,
        ..Default::default()
    }
}

fn generate_fight() -> Room {
    Room {
        door: true,
        description: true,
        search: true,
        combat: true,
        ..Default::default()
    }
}

fn get_enemy() -> Combatant {
    Combatant {
        health: 8,
        proficiency: 0,
        damage_res: 0,
        damage_bonus: 0,
    }
}
