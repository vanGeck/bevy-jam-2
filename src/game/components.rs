use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,

    // 0 means free, 1 means occupied
    pub occupied: Vec<bool>,
}

#[derive(Component, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}