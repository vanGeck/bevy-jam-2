use bevy::prelude::*;
use rand::Rng;

use crate::config::data_sim_texts::DungeonTexts;
use crate::game::dungeonsim::dungeon_components::TextType;

/// Cause a message to be printed and maybe a sound to be played.
pub struct SimMessageEvent(pub TextType);

pub fn push_message(mut events: EventReader<SimMessageEvent>, texts: Res<DungeonTexts>) {
    for SimMessageEvent(text_type) in events.iter() {
        trace!("Received sim message event for TextType::{:?}", text_type);
        let random = pick_random_from_series(&texts.map.get(text_type).unwrap_or(&Vec::new()));
        if let Some(message) = random {
            println!("{}", message);
        } else {
            error!("Missing or empty dungeon text: TextType::{:?}", text_type);
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
