use crate::audio::record_player::animate;
use crate::audio::sound_event::SoundEvent;
use crate::game::combat::{Combatant, Enemy, Hero};
use crate::game::dungeon_sim::{init_dungeon, tick_dungeon};
use crate::game::event_handling::{
    handle_sim_loot, handle_sim_message, SimLootEvent, SimMessageEvent,
};
use crate::game::{
    apply_scrim_to_being_dragged, check_drag_begin, check_drag_end, check_ghost_placement_validity,
    combine_items_system, process_drag_event, set_ghost_position, spawn_item, AlbumId,
    CleanupOnGameplayEnd, CombineButton, DragEvent, Item, ItemId, Player, SoundId, SpawnItemEvent,
    TextureId,
};
use crate::hud::gold::gold_update_system;
use crate::mouse::{reset_cursor, set_cursor_appearance, Mouse};
use crate::positioning::{Coords, Dimens, Pos};
use crate::states::handle_state_transition;
use crate::AppState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use crate::game::item_info_system::*;

use super::{setup_health_bar, update_health_bar, Eyes, Iris};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnItemEvent>()
            .add_event::<DragEvent>()
            .add_event::<SimMessageEvent>()
            .add_event::<SimLootEvent>()
            .init_resource::<Player>()
            .insert_resource(Hero {
                combat_stats: Combatant {
                    health: 20,
                    max_health: 20,
                    proficiency: 1,
                    damage_res: 1,
                    damage_bonus: 0,
                },
            })
            .insert_resource(Enemy {
                combat_stats: Default::default(),
            })
            .add_enter_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    // .with_system(setup)
                    .with_system(init_dungeon)
                    .with_system(create_debug_items)
                    .with_system(setup_health_bar)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(handle_state_transition)
                    .with_system(spawn_item)
                    .with_system(set_cursor_appearance)
                    .with_system(check_drag_begin)
                    .with_system(set_ghost_position)
                    .with_system(apply_scrim_to_being_dragged)
                    .with_system(check_ghost_placement_validity)
                    .with_system(check_drag_end)
                    .with_system(process_drag_event)
                    .with_system(combine_items_system)
                    .with_system(gold_update_system)
                    .with_system(animate)
                    .with_system(track_combine_button_hover)
                    .with_system(tick_dungeon)
                    .with_system(handle_sim_message)
                    .with_system(handle_sim_loot)
                    .with_system(update_health_bar)
                    .with_system(eye_tracking_system)
                    .with_system(item_info_system)
                    .with_system(display_item_info_system)
                    .into(),
            )
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
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

// TODO: Move this to it's own system?
fn setup(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::Music(AlbumId::Jazz));
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

pub fn eye_tracking_system(
    mouse: Res<Mouse>,
    eyes: Query<(&Eyes, &Transform), Without<Iris>>,
    mut iris: Query<(&Iris, &mut Transform), Without<Eyes>>,
) {
    if let Ok((_, white)) = eyes.get_single() {
        if let Ok((_, mut iris)) = iris.get_single_mut() {
            let white_pos = white.translation.truncate();
            let new_iris_trans = white.translation + ((mouse.position - white_pos) / 100.0)
                .clamp_length(0.0, 0.2)
                .extend(1.0);
            iris.translation = new_iris_trans;
        }
    }
}

pub fn track_combine_button_hover(
    mut audio: EventWriter<SoundEvent>,
    input: Res<Input<MouseButton>>,
    mouse: Res<Mouse>,
    mut button: Query<(&mut Sprite, &Transform, &CombineButton)>,
) {
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

pub fn create_debug_items(mut spawn: EventWriter<SpawnItemEvent>) {
    spawn.send(SpawnItemEvent::new(
        Item {
            id: ItemId::Vial,
            texture_id: TextureId::Vial,
            name: "Vial".to_string(),
            description: "A Vial".to_string(),
            wearable: None,
            ..default()
        },
        Coords::new(Pos::new(0, 0), Dimens::new(1, 2)),
    ));
    spawn.send(SpawnItemEvent::new(
        Item {
            id: ItemId::Vial,
            texture_id: TextureId::Vial,
            name: "Vial".to_string(),
            description: "A glass Vial".to_string(),
            wearable: None,
            ..default()
        },
        Coords::new(Pos::new(1, 0), Dimens::new(1, 2)),
    ));
    // spawn.send(SpawnItemEvent::new(
    //     Item {
    //         id: ItemId::EmptyLantern,
    //         texture_id: TextureId::EmptyLantern,
    //         name: "".to_string(),
    //         description: "".to_string(),
    //         wearable: None,
    //         ..default()
    //     },
    //     Coords::new(Pos::new(5, 5), Dimens::new(2, 3)),
    // ));
    spawn.send(SpawnItemEvent::new(
        Item {
            id: ItemId::HerbRed,
            texture_id: TextureId::HerbRed,
            name: "Red Herb".to_string(),
            description: "A herb the color of blood".to_string(),
            wearable: None,
            ..default()
        },
        Coords::new(Pos::new(2, 0), Dimens::new(1, 1)),
    ));
    spawn.send(SpawnItemEvent::new(
        Item {
            id: ItemId::HerbGreen,
            texture_id: TextureId::HerbGreen,
            name: "Green Herb".to_string(),
            description: "A herb the color of nature".to_string(),
            wearable: None,
            ..default()
        },
        Coords::new(Pos::new(2, 1), Dimens::new(1, 1)),
    ));
}
