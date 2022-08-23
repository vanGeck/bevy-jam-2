use bevy::prelude::*;

/// Deprecated
#[derive(Component, Debug)]
pub struct Grid2 {
    pub coords: Coords,
    pub tiles: Vec<Tile>,
}
use crate::hud::gold::Gold;

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
