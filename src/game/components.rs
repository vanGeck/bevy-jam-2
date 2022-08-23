use bevy::prelude::*;

use crate::hud::gold::Gold;

/// === Resources ===
#[derive(Default)]
pub struct Player {
    pub gold: Gold,
}

/// === Components
// Place this component on every gameplay entity that needs to be destroyed when game ends.
#[derive(Component)]
pub struct CleanupOnGameplayEnd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum GameResult {
    Lost,
    Won,
}