use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use bevy::asset::LoadState;
use bevy::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::config::config_audio::AudioConfig;
use crate::config::config_debug::DebugConfig;
use crate::config::config_grid::GridConfig;
use crate::config::config_sim::SimConfig;
use crate::config::data_items::ItemsData;
use crate::config::data_recipes::RecipesData;
use crate::config::data_sim_texts::DungeonTexts;
use crate::game::AssetStorage;
use crate::loading::atlas_prefab::AtlasPrefab;
use crate::loading::config::LoadingConfig;
use crate::{AppState, WindowMode};

pub fn load_configs(mut commands: Commands) {
    commands.insert_resource(GridConfig::load_from_file());
    commands.insert_resource(DebugConfig::load_from_file());
    commands.insert_resource(AudioConfig::load_from_file());
    commands.insert_resource(ItemsData::load_from_file());
    commands.insert_resource(RecipesData::load_from_file());
    commands.insert_resource(DungeonTexts::load_from_file());
    commands.insert_resource(SimConfig::load_from_file());
}

pub fn load_assets(
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut storage: ResMut<AssetStorage>,
) {
    let config = LoadingConfig::load_from_file();
    for (sprite_type, path) in config.atlases {
        let file = PathBuf::new().join("assets/atlases/").join(path);
        let data = fs::read_to_string(&file).expect("Unable to read file");
        let atlas_prefab = ron::de::from_str::<AtlasPrefab>(&data)
            .unwrap_or_else(|_| panic!("Unable to deserialise AtlasPrefab at path {:?}", &file));
        let texture_handle =
            assets.load(PathBuf::new().join("textures/").join(&atlas_prefab.texture));
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            atlas_prefab.tile_size,
            atlas_prefab.columns,
            atlas_prefab.rows,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        storage.put_atlas(sprite_type, texture_atlas_handle);
    }

    for (sprite_type, path) in config.textures {
        let texture_handle = assets.load(PathBuf::new().join("textures/").join(path));
        storage.put_texture(sprite_type, texture_handle);
    }
    for (font_type, path) in config.fonts {
        let font_handle = assets.load(PathBuf::new().join("fonts/").join(path));
        storage.put_font(font_type, font_handle);
    }

    for (sound_type, path) in config.sound_effects {
        let asset_path = PathBuf::new().join("audio/sfx/").join(path);
        let file = PathBuf::new().join("assets/").join(&asset_path);
        if file.is_file() {
            let handle = assets.load(asset_path);
            storage.put_sound(sound_type, handle);
        } else if file.is_dir() {
            for handle in assets.load_folder(asset_path).unwrap() {
                storage.put_sound(sound_type, handle.typed());
            }
        } else {
            warn!("Did not recognise path {:?}", asset_path);
        }
    }

    for (music_type, path) in config.music {
        let asset_path = PathBuf::new().join("audio/music/").join(path.clone());
        let file = PathBuf::new().join("assets/").join(&asset_path);
        if file.is_file() {
            let handle = assets.load(asset_path);
            storage.put_music(music_type, handle, path.clone());
        } else if file.is_dir() {
            for child in get_children(&file) {
                let handle = assets.load(asset_path.join(child.clone()));
                storage.put_music(
                    music_type,
                    handle,
                    child
                        .clone()
                        .file_name()
                        .unwrap_or(&OsStr::new("Filename not found"))
                        .to_str()
                        .unwrap_or("Filename not found")
                        .to_string(),
                );
            }
        } else {
            warn!("Did not recognise path {:?}", asset_path);
        }
    }
}

/// Helper function to collect all direct children of the given directory.
/// This will not return child-directories, only regular files.
fn get_children(parent: &Path) -> Vec<PathBuf> {
    fs::read_dir(parent)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect()
}

pub fn check_load_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    storage: Res<AssetStorage>,
    config: Res<DebugConfig>,
    mut windows: ResMut<Windows>,
) {
    match asset_server.get_group_load_state(storage.get_all_handle_ids()) {
        LoadState::Failed => {
            error!("Failed loading assets!");
        }
        LoadState::Loaded => {
            windows.primary_mut().set_mode(if config.launch_fullscreen {
                WindowMode::BorderlessFullscreen
            } else {
                WindowMode::Windowed
            });
            if config.skip_straight_to_game {
                commands.insert_resource(NextState(AppState::InGame));
            } else {
                commands.insert_resource(NextState(AppState::MainMenu));
            }
        }
        _ => (),
    }
}
