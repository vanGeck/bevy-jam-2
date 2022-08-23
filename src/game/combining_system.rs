use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy::utils::HashMap;

use serde::{Deserialize, Serialize};
use crate::game::Item;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Recipes {
    pub recipes: HashMap<String, Recipe>,
}

impl Recipes {
    // Sorry the parameter names aren't the greatest here, over_item is the item that the dragged_item is currently 'hovering' over.
    pub fn is_valid_recipe(&self, dragged_item: Item, over_item: Item) -> Option<&Recipe> {
        let dragged_item_id = dragged_item.id;
        let over_item_id = over_item.id;

        let mut recipe_has_dragged_item: bool = false;
        let mut recipe_has_over_item: bool = false;

        let mut recipe_clone: &Recipe;

        self.recipes.values().for_each(|recipe| {
            recipe.ingredients.iter().for_each(|ingredient| {
                if ingredient.item_id == dragged_item_id {
                    recipe_has_dragged_item = true;
                }
                if ingredient.item_id == over_item_id {
                    recipe_has_over_item = true;
                }

                if recipe_has_dragged_item && recipe_has_over_item {
                    recipe_clone = recipe;
                }
            });
        });

        if recipe_clone.is_some() {
            Some(recipe_clone)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub result_item_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Ingredient {
    pub item_id: String,
    pub quantity: i32,
}

pub fn setup_recipes(mut commands: Commands) {
    let path = PathBuf::new().join("assets/config/default/recipes.ron");
    let recipes = load_from_path(&path);
    commands.insert_resource(recipes);
}

fn load_from_path(path: &Path) -> Recipes {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<Recipes>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the recipes file from {:?}! Falling back to Recipes::default(). Error: {:?}",
                    path, error
                );
            Recipes::default()
        })
}