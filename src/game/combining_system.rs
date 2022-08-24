use crate::config::data_recipes::RecipesData;
use crate::game::items::Item;
use crate::game::recipes::Recipe;
use crate::positioning::Coords;
use bevy::prelude::*;

use super::items::CraftItem;

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
