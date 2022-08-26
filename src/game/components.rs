use bevy::prelude::*;

use crate::hud::gold::Gold;

#[derive(Default)]
pub struct Player {
    pub gold: Gold,
}

#[derive(Component)]
pub struct Eyes;

#[derive(Component)]
pub struct Iris;

// Place this component on every gameplay entity that needs to be destroyed when game ends.
#[derive(Component)]
pub struct CleanupOnGameplayEnd;
