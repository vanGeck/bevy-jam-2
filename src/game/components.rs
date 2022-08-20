use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Grid {
    pub width: i32,
    pub height: i32,

    // false means free, true means occupied
    pub occupied: Vec<bool>,
}

#[derive(Component, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Shape {
    pub width: i32,
    pub height: i32,
    pub occupied: Vec<Vec<bool>>,
}