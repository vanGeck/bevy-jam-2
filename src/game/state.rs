use std::time::Duration;

use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle};
use iyes_loopless::prelude::*;

use crate::audio::record_player::animate;
use crate::audio::sound_event::SoundEvent;
use crate::game::dungeon_sim::{init_dungeon, manage_continue_prompt, tick_dungeon};
use crate::game::event_handling::{
    handle_sim_loot, handle_sim_message, SimLootEvent, SimMessageEvent,
};
use crate::game::feed::{handle_add_to_feed, position_feed_item, AddFeedItemEvent};
use crate::game::item_info_system::*;
use crate::game::timed_effect::{test_apply_modifier, tick_temporary_modifiers, TimedEffectTicker};
use crate::game::{
    animate_falling_item, apply_silhouette, check_drag_begin, check_drag_end,
    check_ghost_placement_validity, combine_items_system, process_drag_event, set_ghost_position,
    spawn_item, AlbumId, AssetStorage, CleanupOnGameplayEnd, DragEvent, ItemId, Player,
    SpawnItemEvent, TextureId,
};
//use crate::hud::gold::gold_update_system;
use crate::config::data_items::ItemsData;
use crate::mouse::Mouse;
use crate::positioning::{Coords, Pos};
use crate::AppState;

use super::combat::{Combatant, Enemy, Hero};
use super::{consume_item, update_health_bar, update_hero_stats_display, Eyes, Iris};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnItemEvent>()
            .add_event::<DragEvent>()
            .add_event::<AddFeedItemEvent>()
            .add_event::<SimMessageEvent>()
            .add_event::<SimLootEvent>()
            .add_plugin(bevy_ninepatch::NinePatchPlugin::<()>::default())
            .init_resource::<Player>()
            .insert_resource(TimedEffectTicker {
                timer: Timer::new(Duration::from_secs(1), true),
            })
            .insert_resource(Hero {
                combat_stats: Combatant {
                    health: 20,
                    max_health: 20,
                    proficiency: 1,
                    damage_res: 0,
                    damage_bonus: 0,
                },
            })
            .init_resource::<Enemy>()
            .add_enter_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(start_jazz_music)
                    .with_system(init_dungeon)
                    .with_system(create_debug_items)
                    //.with_system(test_slice)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(spawn_item)
                    .with_system(check_drag_begin)
                    .with_system(set_ghost_position)
                    .with_system(apply_silhouette)
                    .with_system(check_ghost_placement_validity)
                    .with_system(check_drag_end)
                    .with_system(process_drag_event)
                    .with_system(combine_items_system)
                    .with_system(animate)
                    .with_system(tick_dungeon)
                    .with_system(tick_temporary_modifiers)
                    .with_system(test_apply_modifier)
                    .with_system(handle_sim_message)
                    .with_system(handle_add_to_feed)
                    .with_system(manage_continue_prompt)
                    .with_system(handle_sim_loot)
                    .with_system(update_health_bar)
                    .with_system(update_hero_stats_display)
                    .with_system(eye_tracking_system)
                    .with_system(update_mouse_over_item_info_system)
                    .with_system(update_mouse_over_item_info_style_position_system)
                    .with_system(position_feed_item)
                    .with_system(consume_item)
                    .with_system(animate_falling_item)
                    .into(),
            )
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    // .with_system(despawn_gameplay_entities)
                    .with_system(clear_gameplay_data)
                    .into(),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum GameResult {
    Lost,
    Won,
}

fn start_jazz_music(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::PlayAlbum(AlbumId::Jazz));
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

fn clear_gameplay_data(mut hero: ResMut<Hero>) {
    hero.combat_stats = Combatant {
        health: 20,
        max_health: 20,
        proficiency: 1,
        damage_res: 0,
        damage_bonus: 0,
    };
}

pub fn eye_tracking_system(
    mouse: Res<Mouse>,
    eyes: Query<(&Eyes, &Transform), Without<Iris>>,
    mut iris: Query<(&Iris, &mut Transform), Without<Eyes>>,
) {
    if let Ok((_, white)) = eyes.get_single() {
        if let Ok((_, mut iris)) = iris.get_single_mut() {
            let white_pos = white.translation.truncate();
            let new_iris_trans = white.translation
                + ((mouse.position - white_pos) / 100.0)
                    .clamp_length(0.0, 0.2)
                    .extend(1.0);
            iris.translation = new_iris_trans;
        }
    }
}

pub fn create_debug_items(mut spawn: EventWriter<SpawnItemEvent>, items_db: Res<ItemsData>) {
    let item = items_db.try_get_item(ItemId::ScrollBasic5).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(3, 3), item.0),
    ));

    let mut item = items_db.try_get_item(ItemId::ScrollBasic7).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(3, 4), item.0),
    ));

    item = items_db.try_get_item(ItemId::Vial).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(0, 0), item.0),
    ));

    item = items_db.try_get_item(ItemId::Vial).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(1, 0), item.0),
    ));

    item = items_db.try_get_item(ItemId::HerbRed).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(2, 0), item.0),
    ));

    item = items_db.try_get_item(ItemId::HerbGreen).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(3, 1), item.0),
    ));
    item = items_db.try_get_item(ItemId::MasterworkSwordOfSpeed).unwrap();
    spawn.send(SpawnItemEvent::without_anim(
        item.1,
        Coords::new(Pos::new(7, 0), item.0),
    ));
    // item = items_db.try_get_item(ItemId::SwordRusty).unwrap();
    // spawn.send(SpawnItemEvent::without_anim(
    //     item.1,
    //     Coords::new(Pos::new(4, 0), item.0),
    // ));
    //
    // item = items_db.try_get_item(ItemId::SwordRusty).unwrap();
    // spawn.send(SpawnItemEvent::without_anim(
    //     item.1,
    //     Coords::new(Pos::new(5, 0), item.0),
    // ));
    //item = items_db.try_get_item(ItemId::ShieldRusty).unwrap();
    //spawn.send(SpawnItemEvent::without_anim(
    //    item.1,
    //    Coords::new(Pos::new(6, 0), item.0),
    //));
    //item = items_db.try_get_item(ItemId::Shield).unwrap();
    //spawn.send(SpawnItemEvent::without_anim(
    //    item.1,
    //    Coords::new(Pos::new(0, 2), item.0),
    //));
    //item = items_db.try_get_item(ItemId::ArmorRusty).unwrap();
    //spawn.send(SpawnItemEvent::without_anim(
    //    item.1,
    //    Coords::new(Pos::new(2, 2), item.0),
    //));
    //item = items_db.try_get_item(ItemId::ArmorRusty).unwrap();
    //spawn.send(SpawnItemEvent::without_anim(
    //    item.1,
    //   Coords::new(Pos::new(4, 2), item.0),
    //));
}

fn test_slice(
    mut commands: Commands,
    assets: Res<AssetStorage>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    // Texture for the base image
    let panel_texture_handle =
        Option::<Handle<Image>>::from(assets.texture(&TextureId::UiPanelTexture));

    if let Some(item) = panel_texture_handle {
        info!("texture present");
        let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(30, 30, 30, 30));

        commands.spawn_bundle(
            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
            // of this entity
            NinePatchBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(300.), Val::Px(500.)),
                    ..Default::default()
                },
                nine_patch_data: bevy_ninepatch::NinePatchData {
                    nine_patch: nine_patch_handle,
                    texture: item,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
    } else {
        error!("texture missing");
    }

    return;
}
