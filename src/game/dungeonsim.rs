mod combat;
mod dungeon_components;
mod dungeon;

use std::ops::Range;
use std::time::Duration;
use bevy::log::{debug, info};
use bevy::prelude::{Res, Time, Timer};
use rand::Rng;
use crate::config::dungeon_params::DungeonParams;
use crate::config::dungeon_texts::DungeonTexts;
use crate::game::dungeonsim::dungeon::generate_level;
use crate::game::dungeonsim::dungeon_components::DungeonLevel;
use crate::ResMut;

#[derive(Default)]
pub struct DungeonState {
    pub current_room_idx: i32,
    pub current_level: Option<DungeonLevel>,
    pub msg_cooldown: Timer,
    pub running: bool,
}

pub fn init_dungeon(params: Res<DungeonParams>, mut state: ResMut<DungeonState>){
    state.current_level = Option::from(generate_level(12, &params));
}

pub fn dungeon_text_test(texts: Res<DungeonTexts>, time: Res<Time>, mut state: ResMut<DungeonState>){
    if state.msg_cooldown.tick(time.delta()).just_finished() {
        let t = pick_random_from_series(&texts.enter_room);
        info!(t);
    }
}

pub fn tick_dungeon(texts: Res<DungeonTexts>, time: Res<Time>, mut state: ResMut<DungeonState>){
    if state.msg_cooldown.tick(time.delta()).just_finished() {
        if let Some(level) = &state.current_level {
            let room = &mut level.rooms[state.current_room_idx as usize];
            room.corridor = false;
        }
    }
}

fn pick_random_from_series(strings: &Vec<String>) -> &String {
    let len = (strings.len() - 1) as i32;
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..len) as usize;
    return &strings[idx];
}

pub fn halt_dungeon_sim(mut state: ResMut<DungeonState>){
    info!("Resuming dungeon sim.");
    state.running = false;
}

pub fn resume_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Halting dungeon sim.");
    state.running = true;
}