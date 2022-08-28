use std::collections::HashMap;
use crate::config::config_sim::SimConfig;
use crate::config::data_enemies::EnemiesData;
use crate::game::combat::{DropTable, Enemy, EnemyId};
use crate::game::sim::dungeon_components::{DungeonLevel, Room};
use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::game::ItemId;
use serde::{Serialize, Deserialize};
use crate::game::dungeon_components::TextType;

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
    pub enemies: Option<HashMap<EnemyId, u32>>,
    pub custom_loot: Option<DropTable>,
    pub custom_flavour: Option<TextType>,
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
    _params: &SimConfig,
    blueprint: &LevelBlueprint,
    mut _cmd: &mut Commands,
    enemies_data: &Res<EnemiesData>,
) -> DungeonLevel {
    let mut rooms = Vec::<Room>::new();
    let mut enemies = Vec::<Enemy>::new();
    let mut loot = Vec::<DropTable>::new();
    let mut rng = rand::thread_rng();

    for segment in &blueprint.segments {
        let room_type = choose_room_type(&segment.types, &mut rng);
        match room_type {
            RoomType::Empty => {
                let mut r = generate_empty();
                if let Some(flavour) = segment.custom_flavour.clone() {
                    r.flavour = Option::<TextType>::from(flavour);
                }
                rooms.push(r);
                enemies.push(Enemy::default());
                if let Some(custom) = &segment.custom_loot {
                    let clone = custom.clone();
                    loot.push(clone);
                } else {
                    info!("Pushing default loot to an empty room.");
                    loot.push(blueprint.default_loot.clone());
                }
            }
            RoomType::Fight => {
                if let Some(enemy_opts) = &segment.enemies{
                    enemies.push(get_enemy(&enemies_data, 
                                           choose_monster_type(&enemy_opts, &mut rng)
                    ));
                } else {
                    error!("Room type is >Fight<, but there's no enemy list supplied!");
                    enemies.push(Enemy::default());
                }
                rooms.push(generate_fight());
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

fn get_enemy(enemies: &Res<EnemiesData>, enemy_id: EnemyId) -> Enemy {
    if let Some(nmy) = enemies.enemies.clone()
        .into_iter()
        .find(|p| p.enemy_id == enemy_id){
        return nmy;
    }
    error!("Error during enemy generation, returning default enemy!");
    return Enemy::default();
}
