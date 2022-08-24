pub mod combat;
pub mod dungeon_components;
pub mod dungeon;

use std::ops::Range;
use std::time::Duration;
use bevy::log::{debug, error, info};
use bevy::prelude::{Commands, Query, Res, Time, Timer, With};
use rand::Rng;
use crate::config::dungeon_params::DungeonParams;
use crate::config::dungeon_texts::DungeonTexts;
use crate::game::dungeonsim::combat::{Combatant, CombatState, Enemy, Hero, process_combat};
use crate::game::dungeonsim::dungeon::generate_level;
use crate::game::dungeonsim::dungeon_components::DungeonLevel;
use crate::ResMut;

#[derive(Default)]
pub struct DungeonState {
    pub current_room_idx: i32,
    pub current_level: Option<DungeonLevel>,
    pub msg_cooldown: Timer,
    pub running: bool,
    pub combat_state: CombatState,
}

pub fn init_dungeon(params: Res<DungeonParams>, mut state: ResMut<DungeonState>, mut cmd: Commands){
    state.current_level = Option::from(generate_level(12, &params, &mut cmd));
}

pub fn tick_dungeon(texts: Res<DungeonTexts>, 
                    time: Res<Time>, 
                    mut state: ResMut<DungeonState>, 
                    mut hero: ResMut<Hero>, 
                    mut enemy: ResMut<Enemy>) {
    if state.running == false { return; }
    if state.msg_cooldown.tick(time.delta()).just_finished() {
        let cbt_state = state.combat_state.clone();
        let current_room_idx = state.current_room_idx as usize;
        if let Some(level) = &mut state.current_level {
            let room = &mut level.rooms[current_room_idx as usize];
            
            if room.init {
                room.init = false;
                enemy.combat_stats = level.enemies[current_room_idx].clone();
            }

            if room.corridor {
                room.corridor = false;
                info!("{}", pick_random_from_series(&texts.corridor));
                return;
            }
            if room.door {
                room.door = false;
                info!("{}", pick_random_from_series(&texts.door));
                return;
            }
            if room.combat {
                if cbt_state == CombatState::Init {
                    info!("{}", pick_random_from_series(&texts.enemy_encounter));
                    state.combat_state = CombatState::InProgress;
                    return;
                } else if cbt_state == CombatState::EnemyDead {
                    info!("{}", pick_random_from_series(&texts.combat_enemy_died));
                    state.combat_state = CombatState::Ended;
                    return;
                } else if cbt_state == CombatState::HeroDead {
                    info!("{}", pick_random_from_series(&texts.combat_hero_died));
                    state.combat_state = CombatState::Ended;
                    state.running = false;
                    // TODO: change state to endgame, hero is dead!
                    return;
                } else if cbt_state == CombatState::InProgress {
                    process_combat(&mut enemy.combat_stats, &mut hero.combat_stats, &mut state.combat_state);
                    return;
                } else if cbt_state == CombatState::Ended {
                    room.combat = false;
                }
            }
            if room.description {
                room.description = false;
                info!("{}", pick_random_from_series(&texts.enter_room));
                return;
            }
            if room.search {
                room.search = false;
                info!("{}", pick_random_from_series(&texts.searching_room));
                room.post_search = true;
                return;
            }
            if room.post_search{
                // TODO:
                // Plugin in an event to spawn items in pack!
                // Plug in loot tables and drop rates.
                // Use halt/resume methods to allow for looting in peace.
                room.post_search = false;
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0..100);
                if x < 35 {
                    info!("{}", pick_random_from_series(&texts.found_loot));
                } else {
                    info!("{}", pick_random_from_series(&texts.found_nothing));
                }
            }
            
            if room.start {
                room.start = false;
                info!("You descend into the darkness of the dungeon...");
                return;
            }
            if room.end {
                room.end = false;
                info!("You've reached the last room. There's a downward staircase here...");
                return;
            }
            
            if level.rooms.len() - 1 > state.current_room_idx as usize {
                state.current_room_idx += 1;
            }
        }
    }
}



fn pick_random_from_series(strings: &Vec<String>) -> &String {
    let len = strings.len() as i32;
    if len == 1 {
        return &strings[0];
    } else if len == 0 {
        panic!("Empty string vector! Check .ron file with dungeon texts!");
    } else {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..len) as usize;
        return &strings[idx];
    }

}

pub fn halt_dungeon_sim(mut state: ResMut<DungeonState>){
    info!("Resuming dungeon sim.");
    state.running = false;
}

pub fn resume_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Halting dungeon sim.");
    state.running = true;
}