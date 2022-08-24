use bevy::prelude::*;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::*;

pub use assets::*;
pub use components::*;
pub use spawn_item_system::*;

use crate::audio::audio::*;
use crate::audio::record_player::animate;
use crate::audio::sound_event::SoundEvent;
use crate::game::camera::create_camera;
use crate::game::create_grid_system::create_grids;
use crate::game::dragging_items_system::{
    check_drag_begin, check_drag_end, process_drag_ended_event, update_dragged_ghost_item_position,
    update_dragged_ghost_item_validity, update_dragged_item_tint, DragEvent,
};
use crate::game::items::{Item, ItemId};
use crate::hud::gold::{setup_gold, update_gold_timer};
use crate::mouse::{reset_mouse, setup_mouse, update_mouse_cursor_icon, update_mouse_pos, Mouse};
use crate::positioning::{Coords, Dimens, Pos};
use crate::AppState;
use combining_system::*;

pub mod assets;
pub mod camera;
pub mod combining_system;
mod components;
mod create_grid_system;
pub mod dragging_items_system;
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
                    .with_system(create_camera)
                    .with_system(create_debug_items)
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
                    .with_system(combine_items_system)
                    .with_system(update_spawn_item_timer)
                    .with_system(update_gold_timer)
                    .with_system(spawn_new_items)
                    .with_system(update_gold_timer)
                    .with_system(animate)
                    .with_system(track_combine_button_hover)
                    .into(),
            )
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(despawn_gameplay_entities)
                    .with_system(reset_mouse)
                    .into(),
            );
    }
}

pub fn create_debug_items(mut spawn: EventWriter<SpawnItemEvent>) {
    spawn.send(SpawnItemEvent::new(
        Item {
            id: ItemId::CandleStick,
            texture_id: TextureId::CandleStick,
            name: "".to_string(),
            description: "".to_string(),
            wearable: None,
        },
        Coords::new(Pos::new(10, 10), Dimens::new(1, 2)),
    ));
    spawn.send(SpawnItemEvent::new(
        Item {
            id: ItemId::EmptyLantern,
            texture_id: TextureId::EmptyLantern,
            name: "".to_string(),
            description: "".to_string(),
            wearable: None,
        },
        Coords::new(Pos::new(5, 5), Dimens::new(2, 3)),
    ));
}

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

fn setup(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::Music(Some((MusicId::Placeholder, false))));
}

pub fn despawn_gameplay_entities(
    mut cmd: Commands,
    mut audio: EventWriter<SoundEvent>,
    query: Query<Entity, With<CleanupOnGameplayEnd>>,
) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
    audio.send(SoundEvent::KillAllMusic);
}

// This feels overkill, with a set window size we could use regular UI entities instead that
// come with the interactions component premade
pub fn track_combine_button_hover(
    mut audio: EventWriter<SoundEvent>,
    input: Res<Input<MouseButton>>,
    query_mouse: Query<&Mouse>,
    mut button: Query<(&mut Sprite, &Transform, &CombineButton)>,
) {
    let mouse = query_mouse.single();
    let mouse_hovers_over_button = button.get_single().map_or(false, |(_, transform, button)| {
        mouse.position.x > transform.translation.x - button.coords.dimens.x as f32 * 0.5
            && mouse.position.x < transform.translation.x + button.coords.dimens.x as f32 * 0.5
            && mouse.position.y > transform.translation.y - button.coords.dimens.y as f32 * 0.5
            && mouse.position.y < transform.translation.y + button.coords.dimens.y as f32 * 0.5
    });

    if mouse_hovers_over_button && input.just_pressed(MouseButton::Left) {
        audio.send(SoundEvent::Sfx(SoundId::Placeholder));
        if let Ok((mut sprite, _, _)) = button.get_single_mut() {
            if mouse_hovers_over_button {
                sprite.color = Color::rgba(255.0, 255.0, 255.0, 0.8);
            }
        }
        // TODO: Check is_valid_recipe with craft_items, combine()
    } else if let Ok((mut sprite, _, _)) = button.get_single_mut() {
        sprite.color = Color::rgba(0.2, 0.2, 0.2, 0.8);
    }
}
