use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub use assets::*;
pub use combining_system::*;
pub use components::*;
pub use spawn_item_system::*;

use crate::audio::record_player::animate;
use crate::audio::sound_event::SoundEvent;
use crate::game::camera::create_camera;
use crate::game::create_grid_system::create_grids;
use crate::game::dragging::{
    apply_scrim_to_being_dragged, check_drag_begin, check_drag_end, check_ghost_placement_validity,
    process_drag_event, set_ghost_position, DragEvent,
};
use crate::hud::gold::{gold_update_system, setup_gold};
use crate::mouse::{reset_cursor, set_cursor_appearance};
use crate::AppState;

pub mod assets;
pub mod camera;
mod combining_system;
mod components;
mod create_grid_system;
pub mod dragging;
pub mod items;
pub mod recipes;
mod spawn_item_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnItemEvent>()
            .add_event::<DragEvent>()
            .init_resource::<Player>()
            .add_enter_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(setup)
                    .with_system(setup_gold)
                    .with_system(setup_spawn_item_timer)
                    .with_system(create_camera)
                    .with_system(create_grids)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(spawn_item_timer_system)
                    .with_system(spawn_item)
                    .with_system(set_cursor_appearance)
                    .with_system(check_drag_begin)
                    .with_system(set_ghost_position)
                    .with_system(apply_scrim_to_being_dragged)
                    .with_system(check_ghost_placement_validity)
                    .with_system(check_drag_end)
                    .with_system(process_drag_event)
                    .with_system(gold_update_system)
                    .with_system(animate)
                    .into(),
            )
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(despawn_gameplay_entities)
                    .with_system(reset_cursor)
                    .into(),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum GameResult {
    Lost,
    Won,
}

fn setup(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::Music(Some((MusicId::Placeholder, false))));
}

pub fn despawn_gameplay_entities(
    mut cmd: Commands,
    mut audio: EventWriter<SoundEvent>,
    q: Query<Entity, With<CleanupOnGameplayEnd>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
    audio.send(SoundEvent::KillAllMusic);
}
