use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AtlasPrefab {
    pub texture: String,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
}
