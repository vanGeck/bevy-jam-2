use bevy::prelude::*;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::*;

pub use assets::*;
pub use components::*;
pub use item_spawner::*;
pub use spawn_item_system::*;

use crate::audio::sound_event::SoundEvent;
use crate::game::camera::create_camera;
use crate::game::dragging::{
    apply_scrim_to_being_dragged, check_drag_begin, check_drag_end, check_ghost_placement_validity,
    process_drag_event, set_ghost_position, DragEvent,
};
use crate::hud::gold::{gold_update_system, setup_gold};
use crate::game::create_grid_system::create_grids;
use crate::mouse::{calc_mouse_pos, configure_cursor, reset_cursor, set_cursor_sprite};
use crate::AppState;

pub mod assets;
pub mod camera;
mod components;
mod dragging;
mod item_spawner;
mod create_grid_system;
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
                    .with_system(configure_cursor)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(draw_win_lose_placeholder_menu)
                    .with_system(spawn_item_system)
                    .with_system(spawn_item)
                    .with_system(calc_mouse_pos)
                    .with_system(set_cursor_sprite)
                    .with_system(check_drag_begin)
                    .with_system(set_ghost_position)
                    .with_system(apply_scrim_to_being_dragged)
                    .with_system(check_ghost_placement_validity)
                    .with_system(check_drag_end)
                    .with_system(process_drag_event)
                    .with_system(gold_update_system)
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
    audio.send(SoundEvent::Music(Some((MusicType::Placeholder, false))));
}

fn draw_win_lose_placeholder_menu(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    mut egui_context: ResMut<EguiContext>,
    mut result: ResMut<State<GameResult>>,
) {
    egui::Window::new("Gameplay").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Win").clicked() {
            audio.send(SoundEvent::Sfx(SoundType::Placeholder));
            commands.insert_resource(NextState(AppState::GameEnded));
            result.replace(GameResult::Won).ok();
        }
        if ui.button("Lose").clicked() {
            audio.send(SoundEvent::Sfx(SoundType::Placeholder));
            commands.insert_resource(NextState(AppState::GameEnded));
            result.replace(GameResult::Lost).ok();
        }
    });
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
