use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use rand::Rng;
use crate::AppState;

use crate::config::config_sim::SimConfig;
use crate::config::data_enemies::EnemiesData;
use crate::config::dungeon_layout::DungeonBlueprint;
use crate::game::combat::{DropTable, EnemyId};
use crate::game::event_handling::SimMessageEvent;
use crate::game::sim::combat::{process_combat, CombatState, Enemy, Hero};
use crate::game::sim::dungeon_gen::generate_level;
use crate::game::sim::dungeon_components::{DungeonLevel, TextType};
use crate::game::sim::event_handling::SimLootEvent;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, FontId, ItemId};

/// Handle a state event. Mainly handle hero's death?
pub struct SimStateEvent(String);

#[derive(Default, Clone)]
pub struct DungeonState {
    pub max_depth: i32,
    pub current_room_idx: i32,
    pub current_level: Option<DungeonLevel>,
    pub msg_cooldown: Timer,
    pub running: bool,
    pub combat_state: CombatState,
}

#[derive(Component)]
pub struct ContinuePrompt;

pub fn init_dungeon(mut commands: Commands, params: Res<SimConfig>, dungeon_bp: Res<DungeonBlueprint>, enemies: Res<EnemiesData>) {
    let mut state = DungeonState {
        max_depth: dungeon_bp.levels.len() as i32 - 1,
        current_room_idx: 0,
        current_level: None,
        msg_cooldown: Timer::new(Duration::from_millis(params.duration_millis), true),
        running: true,
        combat_state: CombatState::Init,
    };
    state.current_level = Option::from(generate_level( &dungeon_bp.levels[0], &mut commands, &enemies));
    commands.insert_resource(state);
}

pub fn progress_dungeon_depth(state: &mut ResMut<DungeonState>, dungeon_bp: Res<DungeonBlueprint>, enemies: Res<EnemiesData>, mut cmd: Commands){
    let next_level_depth = state.clone().current_level.unwrap().depth+1;
    state.current_room_idx = 0;
    state.current_level = Option::from(generate_level(&dungeon_bp.levels[next_level_depth as usize], &mut cmd, &enemies));
    state.combat_state = CombatState::Init;
}

pub fn tick_dungeon(
    mut msg_events: EventWriter<SimMessageEvent>,
    mut loot_events: EventWriter<SimLootEvent>,
    dungeon_bp: Res<DungeonBlueprint>,
    enemy_data: Res<EnemiesData>,
    time: Res<Time>,
    _config: ResMut<SimConfig>,
    mut state: ResMut<DungeonState>,
    mut hero: ResMut<Hero>,
    mut enemy: ResMut<Enemy>,
    input: Res<Input<KeyCode>>,
    mut cmd: Commands,
) {
    let mut just_resumed = false;
    if !state.running {
        if input.just_pressed(KeyCode::Space) && state.combat_state != CombatState::HeroDead{
            state.running = true;
            just_resumed = true;
        } else {
            return;
        }
    }
    if state.msg_cooldown.tick(time.delta()).just_finished() || just_resumed {
        if just_resumed {
            state.msg_cooldown.reset();
        }
        let cbt_state = state.combat_state.clone();
        let current_room_idx = state.current_room_idx.clone() as usize;
        let max_depth = (&state.max_depth).clone();
        if let Some(level) = &mut state.current_level {
            let room = &mut level.rooms[current_room_idx as usize];
            let loot = &mut level.loot[current_room_idx as usize];

            if room.init {
                room.init = false;
                let new_enemy = level.enemies[current_room_idx].clone();
                enemy.combat_stats = new_enemy.combat_stats;
                enemy.enemy_id = new_enemy.enemy_id;
                enemy.enter_combat_text = new_enemy.enter_combat_text;
                enemy.drop_table = new_enemy.drop_table;
                enemy.name = new_enemy.name;
                debug!("New Room: {}", room);
                debug!("Enemy: id: {}, stats: {}", enemy.name, enemy.combat_stats);
            }

            if room.corridor {
                room.corridor = false;
                msg_events.send(SimMessageEvent(TextType::Corridor));
                return;
            }
            if room.door {
                room.door = false;
                msg_events.send(SimMessageEvent(TextType::Door));
                return;
            }
            if room.combat {
                if cbt_state == CombatState::Init {
                    // Monster enounter texts now come from a different source
                    // (each monster has a different one)
                    //.send(SimMessageEvent(TextType::EnemyEncounter));
                    msg_events.send(SimMessageEvent(enemy.enter_combat_text));
                    state.combat_state = CombatState::InProgress;
                    return;
                } else if cbt_state == CombatState::EnemyDead {
                    msg_events.send(SimMessageEvent(TextType::CombatEnemyDied));
                    state.combat_state = CombatState::Ended;
                    return;
                } else if cbt_state == CombatState::HeroDead {
                    msg_events.send(SimMessageEvent(TextType::CombatHeroDied));
                    state.combat_state = CombatState::Ended;
                    halt_dungeon_sim(state);
                    // HERO IS DEAD, END GAME
                    cmd.insert_resource(NextState(AppState::GameEnded));
                    return;
                } else if cbt_state == CombatState::InProgress {
                    process_combat(
                        &mut msg_events,
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
                if let Some(flavour) = room.flavour {
                    msg_events.send(SimMessageEvent(flavour));
                } else {
                    msg_events.send(SimMessageEvent(TextType::EnteredRoom));
                }
                return;
            }

            if room.search {
                if enemy.enemy_id == EnemyId::None {
                    msg_events.send(SimMessageEvent(TextType::SearchingRoom));
                } else {
                    msg_events.send(SimMessageEvent(TextType::SearchingBody));
                }
                room.search = false;
                room.post_search = true;
                return;
            }
            if room.post_search {
                // TODO:
                // Use halt/resume methods to allow for looting in peace.
                room.post_search = false;

                let loot = if enemy.enemy_id == EnemyId::None {
                    info!("Loot pool: {}", &loot.items.len());
                    pick_loot_from_drop_table(&loot)
                } else {
                    info!("Loot pool combat: {}", &enemy.drop_table.items.len());
                    pick_loot_from_drop_table(&enemy.drop_table)
                };
                if loot.len() > 0 {
                    msg_events.send(SimMessageEvent(TextType::FoundLoot));
                    for i in loot {
                        loot_events.send(SimLootEvent(i));
                    }
                } else {
                    msg_events.send(SimMessageEvent(TextType::FoundNothing));
                }
            }

            if room.start {
                room.start = false;
                msg_events.send(SimMessageEvent(TextType::RoomStart));
                return;
            }
            if room.end {
                room.end = false;
                msg_events.send(SimMessageEvent(TextType::RoomEnd));
                return;
            }
            
            if level.rooms.len() - 1 > current_room_idx as usize {
                state.current_room_idx += 1;
                state.combat_state = CombatState::Init;
            } else {
                if level.depth >= max_depth {
                    // GAME ENDED, REACHED LAST ROOM
                    info!("Dungeon complete!");
                    cmd.insert_resource(NextState(AppState::GameEnded));
                    halt_dungeon_sim(state);
                    return;
                } else {
                    // Generate next floor.
                    progress_dungeon_depth(&mut state, dungeon_bp, enemy_data, cmd);
                }
            }
            halt_dungeon_sim(state);
        }
    }
}

pub fn halt_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Halting dungeon sim.");
    state.running = false;
}

pub fn resume_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Resuming dungeon sim.");
    state.running = true;
}

fn pick_loot_from_drop_table(table: &DropTable) -> Vec<ItemId> {
    const MAX_ITEMS: i32 = 3;
    let mut result = Vec::<ItemId>::new();
    let mut rng = rand::thread_rng();
    for i in 0..table.items.len() {
        if result.len() == 3 {
            break;
        }
        let roll = rng.gen_range(1..=100);
        if roll <= table.chances[i] {
            result.push(table.items[i].clone());
        }
    }
    return result;
}

pub fn manage_continue_prompt(state: Res<DungeonState>, q: Query<Entity, With<ContinuePrompt>>, mut cmd: Commands, assets: Res<AssetStorage>){
    if state.running {
        if let Ok(e) = q.get_single() {
            cmd.entity(e).despawn_recursive();
        }
    } else if !state.running && state.combat_state != CombatState::HeroDead {
        if let Ok(_) = q.get_single() {

        } else {
            spawn_prompt(cmd, assets);
        }
    }
}

pub fn spawn_prompt(mut cmd: Commands, assets: Res<AssetStorage>)
{
    cmd.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(16.7)),
            justify_content: JustifyContent::Center,
            position: UiRect{
                left: Val::Percent(44.0),
                right: Val::Percent(40.0),
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).with_children(|parent| {
        // text
        parent.spawn_bundle(
            TextBundle::from_section(
                "Press SPACE to continue exploring!",
                TextStyle {
                    font: assets.font(&FontId::FiraSansBold),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                }),
        );
    }).insert(ContinuePrompt).insert(CleanupOnGameplayEnd);
}