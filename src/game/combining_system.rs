use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::config::data_items::ItemsData;
use crate::config::data_recipes::RecipesData;
use crate::game::items::Item;
use crate::game::recipes::Recipe;
use crate::game::{find_free_space, SpawnItemEvent, SoundId};
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

            let possible_recipe = try_get_recipe(&recipes_data, &items[0], &items[1]);
            trace!("found possible recipe: {:?}", possible_recipe);

            if let Some(recipe) = possible_recipe {
                // debug!("found recipe: {:?}", recipe);
                if let Some((dimens, item)) = items_data.try_get_item(recipe.result) {
                    // debug!("got random item: {:?}", item);

                    if let Some(free_coords) = find_free_space(&grid, dimens, &items_query) {
                        // ^ this is failing
                        debug!("found free space to place the item");
                        // Spawn the result of the recipe
                        spawn_event_writer.send(SpawnItemEvent::new(item, free_coords));
                        // Delete the craft items entities
                        for (entity, _) in crafting_items_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        // Alchemy sound not working or extremely low volume?
                        audio.send(SoundEvent::Sfx(SoundId::Alchemy))
                    } else {
                        warn!("Tried to find free space but failed.");
                    }
                }
            } else {
                audio.send(SoundEvent::Sfx(SoundId::SwordClang))
            }
        }
    }
}

pub fn try_get_recipe(data: &RecipesData, first_item: &Item, second_item: &Item) -> Option<Recipe> {
    let mut recipe_has_first_item: bool = false;
    let mut recipe_has_second_item: bool = false;

    let mut possible_recipe: Option<Recipe> = None;

    if first_item.id == second_item.id {
        return None;
    }

    data.recipes.iter().for_each(|recipe| {
        recipe.ingredients.iter().for_each(|ingredient| {
            if ingredient.item_id == first_item.id {
                recipe_has_first_item = true;
            }
            if ingredient.item_id == second_item.id {
                recipe_has_second_item = true;
            }

            if recipe_has_first_item && recipe_has_second_item {
                possible_recipe = Some(recipe.clone());
            }
        });
        recipe_has_first_item = false;
        recipe_has_second_item = false;
    });

    possible_recipe
}
