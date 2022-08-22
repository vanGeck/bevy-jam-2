use bevy::prelude::*;

use crate::{
    hud::gold::Gold,
    positioning::{coords::Coords, grid::Tile},
};

/// Deprecated
#[derive(Component, Debug)]
pub struct Grid2 {
    pub coords: Coords,
    pub tiles: Vec<Tile>,
}

#[derive(Default)]
pub struct Player {
    pub gold: Gold,
}

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub name: String,
}

// Place this component on every gameplay entity that needs to be destroyed when game ends.
#[derive(Component)]
pub struct CleanupOnGameplayEnd;
