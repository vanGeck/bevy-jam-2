use crate::config::{data_items::ItemsData, data_recipes::RecipesData};
use crate::game::items::Item;
use crate::game::recipes::Recipe;
use crate::positioning::Coords;
use bevy::prelude::*;

use super::{
    items::{CraftItem, ItemId},
    recipes::Ingredient,
    SpawnItemEvent,
};

#[derive(Component)]
pub struct CombineButton {
    pub coords: Coords,
}

pub fn combine(mut commands: Commands, craft_items: Query<Entity, With<CraftItem>>) {
    //TODO: find_free_space in inventory, Spawn the new item there and despawn the craft_items
}

// Sorry the parameter names aren't the greatest here, over_item is the item that the dragged_item is currently 'hovering' over.
pub fn is_valid_recipe(data: &RecipesData, dragged_item: Item, over_item: Item) -> Option<&Recipe> {
    let dragged_item_id = dragged_item.id;
    let over_item_id = over_item.id;

    let mut recipe_has_dragged_item: bool = false;
    let mut recipe_has_over_item: bool = false;

    let mut recipe_clone: Option<&Recipe> = None;

    data.recipes.iter().for_each(|recipe| {
        recipe.ingredients.iter().for_each(|ingredient| {
            if ingredient.item_id == dragged_item_id {
                recipe_has_dragged_item = true;
            }
            if ingredient.item_id == over_item_id {
                recipe_has_over_item = true;
            }

            if recipe_has_dragged_item && recipe_has_over_item {
                recipe_clone = Some(recipe);
            }

            recipe_has_dragged_item = false;
            recipe_has_over_item = false;
        });
    });

    recipe_clone
}

/// === Helpers ===
// Sorry the parameter names aren't the greatest here, over_item is the item that the dragged_item is currently 'hovering' over.
pub fn try_get_recipe(
    data: &RecipesData,
    dragged_item_id: ItemId,
    over_item_id: ItemId,
) -> Option<&Recipe> {
    let mut recipe_has_dragged_item: bool = false;
    let mut recipe_has_over_item: bool = false;

    let mut recipe_clone: Option<&Recipe> = None;

    if dragged_item_id == over_item_id {
        return None;
    }

    data.recipes.iter().for_each(|recipe| {
        recipe.ingredients.iter().for_each(|ingredient| {
            if ingredient.item_id == dragged_item_id {
                recipe_has_dragged_item = true;
            }
            if ingredient.item_id == over_item_id {
                recipe_has_over_item = true;
            }

            if recipe_has_dragged_item && recipe_has_over_item {
                recipe_clone = Some(recipe);
                // is there some way to break out of the for_each loops here?
            }
        });
        recipe_has_dragged_item = false;
        recipe_has_over_item = false;
    });

    recipe_clone
}

/// === Systems ===
pub fn combine_items_system(
    mut commands: Commands,
    recipe_query: Query<(Entity, &Recipe)>,
    ingredient_query: Query<(Entity, &Ingredient, &Coords)>,
    mut spawn: EventWriter<SpawnItemEvent>,
    items_data: Res<ItemsData>,
) {
    if let Ok((recipe_entity, recipe)) = recipe_query.get_single() {
        let result_item_id = &recipe.result;

        if let Ok((_, _, coords)) = ingredient_query.get_single() {
            if let Some((_, item)) = items_data.get_item(result_item_id.clone()) {
                spawn.send(SpawnItemEvent::new(item, coords.clone()));
            }
        }

        commands.entity(recipe_entity).despawn_recursive();
    }

    ingredient_query
        .iter()
        .for_each(|(ingredient_entity, _, _)| {
            commands.entity(ingredient_entity).despawn_recursive();
        });
}
