use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::config::dungeon_params::DungeonParams;
use crate::game::dungeonsim::combat::{process_combat, CombatState, Enemy, Hero};
use crate::game::dungeonsim::dungeon::generate_level;
use crate::game::dungeonsim::dungeon_components::{DungeonLevel, TextType};
use crate::game::dungeonsim::event_handling::SimMessageEvent;

pub mod combat;
pub mod dungeon;
pub mod dungeon_components;
pub mod event_handling;

/// Handle a looting session.
pub struct SimLootEvent;

/// Handle a state event. Mainly handle hero's death?
pub struct SimStateEvent(String);

#[derive(Default)]
pub struct DungeonState {
    pub current_room_idx: i32,
    pub current_level: Option<DungeonLevel>,
    pub msg_cooldown: Timer,
    pub running: bool,
    pub combat_state: CombatState,
}

pub fn init_dungeon(mut commands: Commands, params: Res<DungeonParams>) {
    let mut state = DungeonState {
        current_room_idx: 0,
        current_level: None,
        msg_cooldown: Timer::new(Duration::from_millis(params.duration_millis), true),
        running: true,
        combat_state: CombatState::Init,
    };
    state.current_level = Option::from(generate_level(12, &params, &mut commands));
    commands.insert_resource(state);
}

pub fn tick_dungeon(
    mut events: EventWriter<SimMessageEvent>,
    time: Res<Time>,
    mut state: ResMut<DungeonState>,
    mut hero: ResMut<Hero>,
    mut enemy: ResMut<Enemy>,
) {
    if state.running == false {
        return;
    }
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
                events.send(SimMessageEvent(TextType::Corridor));
                return;
            }
            if room.door {
                room.door = false;
                events.send(SimMessageEvent(TextType::Door));
                return;
            }
            if room.combat {
                if cbt_state == CombatState::Init {
                    events.send(SimMessageEvent(TextType::EnemyEncounter));
                    state.combat_state = CombatState::InProgress;
                    return;
                } else if cbt_state == CombatState::EnemyDead {
                    events.send(SimMessageEvent(TextType::CombatEnemyDied));
                    state.combat_state = CombatState::Ended;
                    return;
                } else if cbt_state == CombatState::HeroDead {
                    events.send(SimMessageEvent(TextType::CombatHeroDied));
                    state.combat_state = CombatState::Ended;
                    state.running = false;
                    // TODO: change state to endgame, hero is dead!
                    return;
                } else if cbt_state == CombatState::InProgress {
                    process_combat(
                        &mut events,
                        &mut enemy.combat_stats,
                        &mut hero.combat_stats,
                        &mut state.combat_state,
                    );
                    return;
                } else if cbt_state == CombatState::Ended {
                    room.combat = false;
                }
            }
            if room.description {
                room.description = false;
                events.send(SimMessageEvent(TextType::EnteredRoom));
                return;
            }
            if room.search {
                room.search = false;
                events.send(SimMessageEvent(TextType::SearchingRoom));
                room.post_search = true;
                return;
            }
            if room.post_search {
                // TODO:
                // Plugin in an event to spawn items in pack!
                // Plug in loot tables and drop rates.
                // Use halt/resume methods to allow for looting in peace.
                room.post_search = false;
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0..100);
                if x < 35 {
                    events.send(SimMessageEvent(TextType::FoundLoot));
                } else {
                    events.send(SimMessageEvent(TextType::FoundNothing));
                }
            }

            if room.start {
                room.start = false;
                events.send(SimMessageEvent(TextType::RoomStart));
                return;
            }
            if room.end {
                room.end = false;
                events.send(SimMessageEvent(TextType::RoomEnd));
                return;
            }

            if level.rooms.len() - 1 > state.current_room_idx as usize {
                state.current_room_idx += 1;
            }
        }
    }
}

pub fn halt_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Resuming dungeon sim.");
    state.running = false;
}

pub fn resume_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Halting dungeon sim.");
    state.running = true;
}
