use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy::utils::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Recipes {
    pub recipes: HashMap<String, Recipe>,
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