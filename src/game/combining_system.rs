use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::config::data_items::ItemsData;
use crate::config::data_recipes::RecipesData;
use crate::game::items::Item;
use crate::game::recipes::Recipe;
use crate::game::{find_free_space, ItemId, SoundId, SpawnItemEvent};
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, GridData};

use super::items::CraftItem;

#[derive(Component)]
pub struct CombineButton {
    pub coords: Coords,
}

// use events here so this doesn't run once a frame?
pub fn combine_items_system(
    mut commands: Commands,
    mut spawn_event_writer: EventWriter<SpawnItemEvent>,
    mut audio: EventWriter<SoundEvent>,
    recipes_data: Res<RecipesData>,
    items_data: Res<ItemsData>,
    grid: Res<GridData>,
    combine_button_query: Query<&MouseInteractive, With<CombineButton>>,
    crafting_items_query: Query<(Entity, &Item), With<CraftItem>>,
    items_query: Query<&Coords, With<Item>>,
) {
    if let Ok(combine_button) = combine_button_query.get_single() {
        if combine_button.clicked {
            let number_of_crafting_items = crafting_items_query.iter().count();
            if number_of_crafting_items <= 1 {
                return;
            }

            let mut items = Vec::new();
            for (_, item) in crafting_items_query.iter() {
                items.push(item.clone());
            }

            let possible_recipe = try_get_recipe(&recipes_data, &items);
            trace!("found possible recipe: {:?}", possible_recipe);

            if let Some(recipe) = possible_recipe {
                // debug!("found recipe: {:?}", recipe);
                if let Some((dimens, item)) = items_data.try_get_item(recipe.result) {
                    // debug!("got random item: {:?}", item);

                    if let Some(free_coords) = find_free_space(&grid, dimens, &items_query, &[]) {
                        // ^ this is failing
                        debug!("found free space to place the item");
                        // Spawn the result of the recipe
                        spawn_event_writer.send(SpawnItemEvent::new(
                            item,
                            free_coords,
                            grid.center_crafting(),
                            true,
                        ));
                        // Delete the craft items entities
                        for (entity, _) in crafting_items_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        // Alchemy sound not working or extremely low volume?
                        audio.send(SoundEvent::Sfx(SoundId::CombineAlchemy))
                    } else {
                        warn!("Tried to find free space but failed.");
                    }
                }
            } else {
                audio.send(SoundEvent::Sfx(SoundId::CombineCant))
            }
        }
    }
}

pub fn try_get_recipe(data: &RecipesData, items: &Vec<Item>) -> Option<Recipe> {
    let mut possible_recipe: Option<Recipe> = None;

    let mut flat_recipe = Vec::<ItemId>::new();
    let items_ids: Vec<ItemId> = items.into_iter().map(|f| f.id.clone()).collect();

    for recipe in &data.recipes {
        flat_recipe.clear();
        for ingr in &recipe.ingredients {
            for _ in 0..(ingr.quantity) {
                flat_recipe.push(ingr.item_id.clone());
            }
        }
        let difference: Vec<_> = items_ids
            .clone()
            .into_iter()
            .filter(|item| !flat_recipe.contains(item))
            .collect();
        if difference.len() == 0 {
            possible_recipe = Option::from(recipe.clone());
            break;
        }
    }

    possible_recipe
}
