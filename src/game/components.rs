use bevy::prelude::*;

use crate::grid::coords::Coords;

#[derive(Component, Debug)]
pub struct Grid {
    pub coords: Coords,
    pub tiles: Vec<Tile>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Available,
    Unavailable,
}

#[derive(Component, Debug, Clone)]
pub struct Tile {
    index: usize,
    coords: Coords,
    tile_type: TileType,
}

impl Tile {
    pub fn new(index: usize, coords: Coords, tile_type: TileType) -> Self {
        Tile {
            index,
            coords,
            tile_type,
        }
    }
}

#[derive(Component, Clone)]
pub struct Item {
    pub name: String,
    pub coords: Coords,
}
