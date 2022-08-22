use std::fs;
use std::path::PathBuf;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::NextState;

use crate::config::config_audio::AudioConfig;
use crate::config::config_debug::DebugConfig;
use crate::config::config_grid::GridConfig;
use crate::game::{AssetStorage, SpriteType};
use crate::input::Mouse;
use crate::loading::atlas_prefab::AtlasPrefab;
use crate::loading::config::LoadingConfig;
use crate::positioning::depth::Depth;
use crate::positioning::dimens::Dimens;
use crate::AppState;

// This is a global look for egui
pub fn configure_ui_look(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn configure_cursor(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    assets: Res<AssetStorage>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Dimens::unit().as_vec2()),
                ..default()
            },
            texture: assets.texture(&SpriteType::Cursor),
            transform: Transform::from_xyz(0., 0., Depth::Cursor.z()),
            ..Default::default()
        })
        .insert(Name::new("MouseCursor"))
        .insert(Mouse::default());
}

pub fn load_configs(mut commands: Commands) {
    commands.insert_resource(GridConfig::load_from_file());
    commands.insert_resource(DebugConfig::load_from_file());
    commands.insert_resource(AudioConfig::load_from_file());
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
        let asset_path = PathBuf::new().join("audio/music/").join(path);
        let file = PathBuf::new().join("assets/").join(&asset_path);
        if file.is_file() {
            let handle = assets.load(asset_path);
            storage.put_music(music_type, handle);
        } else if file.is_dir() {
            for handle in assets.load_folder(asset_path).unwrap() {
                storage.put_music(music_type, handle.typed());
            }
        } else {
            warn!("Did not recognise path {:?}", asset_path);
        }
    }
}

pub fn check_load_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    storage: Res<AssetStorage>,
    config: Res<DebugConfig>,
) {
    match asset_server.get_group_load_state(storage.get_all_handle_ids()) {
        LoadState::Failed => {
            error!("Failed loading assets!");
        }
        LoadState::Loaded => {
            if config.skip_straight_to_game {
                commands.insert_resource(NextState(AppState::InGame));
            } else {
                commands.insert_resource(NextState(AppState::MainMenu));
            }
        }
        _ => (),
    }
}
