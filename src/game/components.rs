use bevy::prelude::*;

use crate::{
    hud::gold::Gold,
    positioning::{coordinates::Coordinates, grid::Tile},
};

/// Deprecated
#[derive(Component, Debug)]
pub struct Grid2 {
    pub coords: Coordinates,
    pub tiles: Vec<Tile>,
}

#[derive(Default)]
pub struct Player {
    pub gold: Gold,
}

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub sprite_path: String,
    pub width: i32,
    pub height: i32,
}

// Place this component on every gameplay entity that needs to be destroyed when game ends.
#[derive(Component)]
pub struct CleanupOnGameplayEnd;
