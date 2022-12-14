use bevy::prelude::*;
use rand::Rng;

use crate::audio::sound_event::SoundEvent;
use crate::config::data_items::ItemsData;
use crate::config::data_layout::LayoutData;
use crate::config::data_texts::TextsData;
use crate::game::dungeon_components::TextType;
use crate::game::feed::AddFeedItemEvent;
use crate::game::{find_free_space, FontId, Item, ItemId, SoundId, SpawnItemEvent};
use crate::positioning::{Coords, GridData};

/// Handle a looting session.
pub struct SimLootEvent(pub ItemId);

pub fn handle_sim_loot(
    mut events: EventReader<SimLootEvent>,
    grid: Res<GridData>,
    layout: Res<LayoutData>,
    items_data: Res<ItemsData>,
    items_query: Query<&Coords, With<Item>>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    let mut same_tick_items: Vec<Coords> = Vec::new();
    for SimLootEvent(item_id) in events.iter() {
        trace!("Received sim loot event");
        if let Some((dimens, item)) = items_data.try_get_item(item_id.clone()) {
            let source = Vec2::new(layout.screen_dimens.x * 0.5, layout.screen_dimens.y + 1.);
            let free_coords = find_free_space(&grid, dimens, &items_query, &same_tick_items);
            if let Some(coords) = free_coords {
                if same_tick_items.contains(&coords) {
                    let new_free_coords =
                        find_free_space(&grid, dimens, &items_query, &same_tick_items);
                    if let Some(coords) = new_free_coords {
                        spawn.send(SpawnItemEvent::new(item, coords, source, false));
                    }
                } else {
                    same_tick_items.push(coords);
                    spawn.send(SpawnItemEvent::new(item, coords, source, false));
                }
            }
        }
    }
}

/// Cause a message to be printed and maybe a sound to be played.
pub struct SimMessageEvent(pub TextType);

pub fn handle_sim_message(
    mut reader: EventReader<SimMessageEvent>,
    mut write_texts: EventWriter<AddFeedItemEvent>,
    mut write_audio: EventWriter<SoundEvent>,
    texts: Res<TextsData>,
) {
    for SimMessageEvent(text_type) in reader.iter() {
        trace!("Received sim message event for TextType::{:?}", text_type);
        let random = pick_random_from_series(texts.map.get(&text_type).unwrap_or(&Vec::new()));
        let colour = text_type.colour_hint();
        let font = if colour.is_major() {
            FontId::FiraSansBold
        } else if colour.is_minor() {
            FontId::FiraSansMedium
        } else {
            FontId::FiraSansRegular
        };
        if let Some(message) = random {
            write_texts.send(AddFeedItemEvent {
                message,
                colour,
                font,
            });
        } else {
            error!("Missing or empty dungeon text: TextType::{:?}", text_type);
        }
        let sfx = match text_type {
            TextType::EnterRat => Some(SoundId::EnterRat),
            TextType::EnterGoblinBrat => Some(SoundId::EnterLittleMonster),
            TextType::EnterGoblinSwordsman => Some(SoundId::EnterBigMonster),
            TextType::EnterGoblinShieldBearer => Some(SoundId::EnterBigMonster),
            TextType::EnterSkeleton => Some(SoundId::EnterSkeleton),
            TextType::EnterZombie => Some(SoundId::EnterZombie),
            TextType::CombatHeroHit => Some(SoundId::SlashHit),
            TextType::CombatEnemyHit => Some(SoundId::SlashHit),
            TextType::CombatHeroDied => Some(SoundId::SlashHit),
            TextType::CombatEnemyDied => Some(SoundId::SlashHit),
            TextType::CombatNoResolution => Some(SoundId::SwordClang),
            TextType::Door => Some(SoundId::DoorCreak),
            _ => None,
        };
        if let Some(sfx) = sfx {
            write_audio.send(SoundEvent::Sfx(sfx));
        }
    }
}

fn pick_random_from_series(strings: &Vec<String>) -> Option<String> {
    if strings.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..strings.len()) as usize;
        strings.get(idx).cloned()
    }
}
