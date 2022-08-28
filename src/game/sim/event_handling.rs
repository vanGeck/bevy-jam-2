use bevy::prelude::*;
use rand::Rng;

use crate::audio::sound_event::SoundEvent;
use crate::config::data_items::ItemsData;
use crate::config::data_sim_texts::DungeonTexts;
use crate::game::dungeon_components::TextType;
use crate::game::feed::AddFeedItemEvent;
use crate::game::{find_free_space, Item, ItemId, SoundId, SpawnItemEvent};
use crate::positioning::{Coords, GridData};

/// Handle a looting session.
pub struct SimLootEvent(pub ItemId);

pub fn handle_sim_loot(
    mut events: EventReader<SimLootEvent>,
    grid: Res<GridData>,
    items_data: Res<ItemsData>,
    items_query: Query<&Coords, With<Item>>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    let mut same_tick_items: Vec<Coords> = Vec::new();
    for SimLootEvent(item_id) in events.iter() {
        trace!("Received sim loot event");
        if let Some((dimens, item)) = items_data.try_get_item(item_id.clone()) {
            let free_coords = find_free_space(&grid, dimens, &items_query, &same_tick_items);
            if same_tick_items.contains(&free_coords.unwrap()) {
                let new_free_coords =
                    find_free_space(&grid, dimens, &items_query, &same_tick_items);
                if let Some(coords) = new_free_coords {
                    spawn.send(SpawnItemEvent::new(item, coords));
                }
            } else if let Some(coords) = free_coords {
                same_tick_items.push(coords);
                spawn.send(SpawnItemEvent::new(item, coords));
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
    texts: Res<DungeonTexts>,
) {
    for SimMessageEvent(text_type) in reader.iter() {
        trace!("Received sim message event for TextType::{:?}", text_type);
        let random = pick_random_from_series(texts.map.get(&text_type).unwrap_or(&Vec::new()));
        if let Some(message) = random {
            write_texts.send(AddFeedItemEvent(message));
        } else {
            error!("Missing or empty dungeon text: TextType::{:?}", text_type);
        }
        let sfx = match text_type {
            TextType::EnterGoblinBrat => Some(SoundId::LittleMonsterGrowl),
            TextType::EnterGoblinSwordsman => Some(SoundId::BigMonsterGrowl),
            TextType::EnterGoblinShieldBearer => Some(SoundId::BigMonsterGrowl),
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
