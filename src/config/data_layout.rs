use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::game::items::EquipmentSlot;
use crate::positioning::Coords;

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct LayoutData {
    /// Divide the layout coordinates by this number.
    pub factor: f32,
    pub text_factor: f32,
    pub screen_dimens: Vec2,
    pub c_left: ColumnLeft,
    pub c_mid: ColumnMiddle,
    pub c_right: ColumnRight,
    /// A number between 0 and 1, describing the height of the baseline in the overseer image.
    pub overseer_baseline: f32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ColumnLeft {
    pub margin_left: f32,
    pub margin_right: f32,
    pub music: Container,
    pub feed: Container,
}

impl ColumnLeft {
    /// Calculate the y position of the music widget.
    pub fn music_y(&self) -> f32 {
        self.music.margin_bottom.unwrap_or(0.)
    }
    /// Calculate the height of the music widget.
    pub fn music_height(&self) -> f32 {
        self.music.height.unwrap()
    }
    /// Calculate the y-position of the dungeon feed widget.
    pub fn feed_y(&self) -> f32 {
        let music_y = self.music_y();
        let music_height = self.music_height();
        music_y + music_height + self.feed.margin_bottom.unwrap_or(0.)
    }
    /// Calculate the height of the dungeon feed widget.
    pub fn feed_height(&self, layout: &LayoutData) -> f32 {
        layout.screen_dimens.y - (self.feed.margin_top.unwrap_or(0.) + self.feed_y())
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ColumnMiddle {
    pub x: f32,
    pub width: f32,
    pub toasts: Container,
    pub inventory: Container,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ColumnRight {
    pub margin_left: f32,
    pub margin_right: f32,
    pub foo: Container,
    pub crafting: Container,
    pub hero: Container,
}

impl ColumnRight {
    /// Calculate the y position of the foo widget.
    pub fn foo_y(&self) -> f32 {
        self.foo.margin_bottom.unwrap_or(0.)
    }
    /// Calculate the height of the foo widget.
    pub fn foo_height(&self) -> f32 {
        self.foo.height.unwrap()
    }
    /// Calculate the y-position of the crafting window.
    pub fn crafting_y(&self) -> f32 {
        let foo_y = self.foo_y();
        let foo_height = self.foo_height();
        foo_y + foo_height + self.crafting.margin_bottom.unwrap_or(0.)
    }
    /// Calculate the height of the crafting window.
    pub fn crafting_height(&self) -> f32 {
        self.crafting.height.unwrap()
    }
    /// Calculate the y-position of the hero widget.
    pub fn hero_y(&self) -> f32 {
        let crafting_y = self.crafting_y();
        let crafting_height = self.crafting_height();
        crafting_y + crafting_height + self.hero.margin_bottom.unwrap_or(0.)
    }
    /// Calculate the height of the hero widget.
    pub fn hero_height(&self, layout: &LayoutData) -> f32 {
        layout.screen_dimens.y - (self.hero.margin_top.unwrap_or(0.) + self.hero_y())
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Container {
    pub margin_bottom: Option<f32>,
    pub margin_top: Option<f32>,
    pub height: Option<f32>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct EquipmentGrid {
    /// The absolute coordinates of the equipment grid. Coordinates of each of the individual slots
    /// are relative to this.
    pub coords: Coords,
    pub slots: HashMap<EquipmentSlot, Coords>,
}

impl LayoutData {
    /// Returns the x position of the left column.
    pub fn left_x(&self) -> f32 {
        self.c_left.margin_left
    }
    /// Returns the width of the left column.
    pub fn left_width(&self) -> f32 {
        self.c_mid.x - self.c_left.margin_left - self.c_left.margin_right
    }
    /// Returns the x position of the middle column.
    pub fn middle_x(&self) -> f32 {
        self.c_mid.x
    }
    /// Returns the width of the middle column.
    pub fn middle_width(&self) -> f32 {
        self.c_mid.width
    }
    /// Returns the x position of the right column.
    pub fn right_x(&self) -> f32 {
        self.middle_x() + self.middle_width() + self.c_right.margin_left
    }
    /// Returns the width of the right column.
    pub fn right_width(&self) -> f32 {
        self.screen_dimens.x - self.c_right.margin_right - self.right_x()
    }

    /// Loads the most relevant instance of `GridConfig`.
    ///
    /// If the `GridConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `GridConfig::default()`).
    ///
    /// If the 'GridConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> LayoutData {
        let override_file = get_config_override_dir().join("layout.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("layout.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> LayoutData {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<LayoutData>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the grid config file from {:?}! Falling back to GridConfig::default(). Error: {:?}",
                    path, error
                );
            LayoutData::default()
        })
}
