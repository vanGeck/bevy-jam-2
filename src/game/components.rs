use bevy::prelude::*;

use crate::grid::coords::Coords;

#[derive(Component, Debug)]
pub struct Grid {
    pub coords: Coords,

    // false means free, true means occupied
    pub occupied: Vec<bool>,
}

#[derive(Component, Clone)]
pub struct Item {
    pub name: String,
    pub coords: Coords,
    pub occupied: Vec<Vec<bool>>,
}
