use std::collections::HashMap;
use crate::config::config_sim::SimConfig;
use crate::config::data_enemies::EnemiesData;
use crate::game::combat::{DropTable, Enemy, EnemyId};
use crate::game::sim::dungeon_components::{DungeonLevel, Room};
use crate::game::ItemId::*;
use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::game::ItemId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DungeonBlueprint {
    pub levels:Vec<LevelBlueprint>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelBlueprint {
    pub depth:i32,
    pub default_loot:DropTable,
    pub segments: Vec<SegmentBlueprint>
}

/// Base building block for the .ron dungeon designs
/// Contains possible room types, custom loot, custom flavour texts, and monster spawn rates.
/// One "segment" results in one room generated.
/// Enemy and room spawn percentages must add up to 100.
/// NOTE: Custom loot works only in empty rooms. Corridors don't yield loot, enemies have their own loot.
#[derive(Serialize, Deserialize, Clone)]
pub struct SegmentBlueprint {
    pub types: HashMap<RoomType, u32>,
    pub enemies: HashMap<EnemyId, u32>,
    pub custom_loot: Option<DropTable>,
    pub custom_flavour: Option<String>,
}

#[derive(Default, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum RoomType {
    #[default]
    Empty,
    Fight,
    Corridor,
    Start,
    End
}

pub fn generate_level(
    len: i32,
    params: &SimConfig,
    //blueprint: &LevelBlueprint,
    mut _cmd: &mut Commands,
    enemies: &Res<EnemiesData>,
) -> DungeonLevel {
    let mut rooms = Vec::<Room>::new();
    let mut enemies = Vec::<Enemy>::new();
    let mut loot = Vec::<DropTable>::new();
    let mut rng = rand::thread_rng();
    
    let blueprint = LevelBlueprint{
        depth: 0,
        default_loot: Default::default(),
        segments: vec![]
    };

    for segment in &blueprint.segments {
        let room_type = choose_room_type(&segment.types, &mut rng);
        match room_type {
            RoomType::Empty => {
                rooms.push(generate_empty());
                enemies.push(Enemy::default());
                if let Some(custom) = &segment.custom_loot {
                    loot.push(custom.clone())
                } else {
                    loot.push(blueprint.default_loot.clone())
                }
            }
            RoomType::Fight => {                
                rooms.push(generate_corridor());
                enemies.push(Enemy::default());
                loot.push(blueprint.default_loot.clone())}
            RoomType::Corridor => {
                rooms.push(generate_corridor());
                enemies.push(Enemy::default());
                loot.push(blueprint.default_loot.clone())
            }
            RoomType::Start => { 
                rooms.push(generate_first_room());
                enemies.push(Enemy::default());
                loot.push(blueprint.default_loot.clone())
            }
            RoomType::End => {
                rooms.push(generate_last_room());
                enemies.push(Enemy::default());
                loot.push(blueprint.default_loot.clone())
            }
        } 
    }
    
    rooms.push(generate_first_room());
    enemies.push(Enemy::default());

    rooms.push(generate_last_room());
    enemies.push(Enemy::default());

    info!("Dungeon generation results: ");
    for s in 0..rooms.len() {
        rooms[s].print_diag_name();
    }

    DungeonLevel {
        depth: 0,
        rooms,
        enemies,
        loot
    }
}

// The choose_x_type methods should be remade to use generics
// I don't understand rust's generics enough - Festus
fn choose_room_type(input: &HashMap<RoomType, u32>, rng: &mut ThreadRng) -> RoomType {
    let roll = rng.gen_range(1..=100);
    let mut tracked_total_perc:u32 = 0;
    for key in input.keys().clone(){
        tracked_total_perc += input.get(key).unwrap();
        if roll <= tracked_total_perc {
            return key.clone();
        }
    }
    return RoomType::default()
}

fn choose_monster_type(input: &HashMap<EnemyId, u32>, rng: &mut ThreadRng) -> EnemyId {
    let roll = rng.gen_range(1..=100);
    let mut tracked_total_perc = 0;
    for key in input.keys().clone(){
        tracked_total_perc += input.get(key).unwrap();
        if roll <= tracked_total_perc {
            return key.clone();
        }
    }
    return EnemyId::default()
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
        search: true,
        combat: true,
        ..Default::default()
    }
}

fn get_enemy(enemies: &Res<EnemiesData>) -> Enemy {
    let enemies_len = enemies.enemies.len();
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..enemies_len);

    enemies.enemies[idx].clone()
}
