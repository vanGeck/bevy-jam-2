use bevy::prelude::*;

use crate::grid::{coords::Coords, tiles::Tile};

#[derive(Component, Debug)]
pub struct Grid {
    pub coords: Coords,
    pub tiles: Vec<Tile>,
}

#[derive(Component, Clone)]
pub struct Item {
    pub name: String,
    pub coords: Coords,
}
