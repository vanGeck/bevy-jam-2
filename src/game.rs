use bevy::prelude::*;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::*;

pub use assets::*;
pub use combining_system::*;
pub use components::*;
pub use spawn_item_system::*;

use crate::audio::sound_event::SoundEvent;
use crate::audio::audio::*;
use crate::game::camera::create_camera;
use crate::game::create_grid_system::create_grids;
use crate::game::dragging::{
    update_dragged_item_tint,
    check_drag_begin,
    check_drag_end,
    update_dragged_ghost_item_validity,
    process_drag_ended_event,
    update_dragged_ghost_item_position,
    DragEndedEvent,
};
use crate::hud::gold::{
    update_gold_timer,
    setup_gold};
use crate::mouse::{
    update_mouse_pos,
    setup_mouse,
    reset_mouse,
    update_mouse_cursor_icon};
use crate::AppState;

pub mod assets;
pub mod camera;
mod combining_system;
mod components;
mod create_grid_system;
mod dragging;
pub mod items;
pub mod recipes;
mod spawn_item_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnItemEvent>()
            .add_event::<DragEndedEvent>()
            .init_resource::<Player>()
            .add_enter_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(create_camera)
                    .with_system(create_grids)
                    .with_system(setup_audio)
                    .with_system(setup_gold)
                    .with_system(setup_mouse)
                    .with_system(setup_spawn_item_timer)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(draw_win_lose_debug_menu)
                    .with_system(update_mouse_pos)
                    .with_system(update_mouse_cursor_icon)
                    .with_system(check_drag_begin)
                    .with_system(update_dragged_ghost_item_position)
                    .with_system(update_dragged_ghost_item_validity)
                    .with_system(update_dragged_item_tint)
                    .with_system(check_drag_end)
                    .with_system(process_drag_ended_event)
                    .with_system(update_spawn_item_timer)
                    .with_system(update_gold_timer)
                    .with_system(spawn_new_items)
                    .into(),
            )
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(cleanup_gameplay_entities)
                    .with_system(reset_mouse)
                    .into(),
            );
    }
}

/// === Systems ===
fn draw_win_lose_debug_menu(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    mut egui_context: ResMut<EguiContext>,
    mut result: ResMut<State<GameResult>>,
) {
    egui::Window::new("Gameplay").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Win").clicked() {
            audio.send(SoundEvent::Sfx(SoundId::Placeholder));
            commands.insert_resource(NextState(AppState::GameEnded));
            result.replace(GameResult::Won).ok();
        }
        if ui.button("Lose").clicked() {
            audio.send(SoundEvent::Sfx(SoundId::Placeholder));
            commands.insert_resource(NextState(AppState::GameEnded));
            result.replace(GameResult::Lost).ok();
        }
    });
}

pub fn cleanup_gameplay_entities(
    mut cmd: Commands,
    mut audio: EventWriter<SoundEvent>,
    query: Query<Entity, With<CleanupOnGameplayEnd>>,
) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
    audio.send(SoundEvent::KillAllMusic);
}
