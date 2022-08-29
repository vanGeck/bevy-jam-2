use std::path::PathBuf;

use bevy::asset::LoadState;
use bevy::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::AppState;
use crate::config::config_audio::AudioConfig;
use crate::config::config_debug::DebugConfig;
use crate::config::config_sim::SimConfig;
use crate::config::data_enemies::EnemiesData;
use crate::config::data_items::ItemsData;
use crate::config::data_layout::LayoutData;
use crate::config::data_recipes::RecipesData;
use crate::config::data_sim_texts::DungeonTexts;
use crate::config::dungeon_layout::DungeonBlueprint;
use crate::game::{AlbumId, AssetStorage, TextureId};
use crate::loading::loading_instructions::{ prepare_loading_config};

pub fn load_configs(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut assets: ResMut<AssetStorage>,
) {
    // commands.insert_resource(DebugConfig::load_from_file());
    // commands.insert_resource(AudioConfig::load_from_file());
    commands.insert_resource(ItemsData::load_from_file());
    commands.insert_resource(RecipesData::load_from_file());
    commands.insert_resource(DungeonTexts::load_from_file());
    commands.insert_resource(SimConfig::load_from_file());
    commands.insert_resource(EnemiesData::load_from_file());
    commands.insert_resource(LayoutData::load_from_file());
    commands.insert_resource(DungeonBlueprint::load_from_file());

    assets.audio = server.load("config/default/config.audio.ron");
    assets.debug = server.load("config/default/config.debug.ron");
}

pub fn load_assets(
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut storage: ResMut<AssetStorage>,
) {
    let config = prepare_loading_config();

    let handle_backpack = assets.load("textures/sheet_backpack.png");
    let atlas_backpack = TextureAtlas::from_grid(handle_backpack, Vec2::new(320.0, 320.0), 3, 1);
    let handle_backpack = texture_atlases.add(atlas_backpack);
    storage.put_atlas(TextureId::Backpack, handle_backpack);

    let handle_player = assets.load("textures/sheet_record_player.png");
    let atlas_player = TextureAtlas::from_grid(handle_player, Vec2::new(640.0, 640.0), 1, 1);
    let handle_player = texture_atlases.add(atlas_player);
    storage.put_atlas(TextureId::RecordPlayer, handle_player);

    for (sprite_type, path) in config.textures {
        let texture_handle = assets.load(PathBuf::new().join("textures/").join(path));
        storage.put_texture(sprite_type, texture_handle);
    }
    for (font_type, path) in config.fonts {
        let font_handle = assets.load(PathBuf::new().join("fonts/").join(path));
        storage.put_font(font_type, font_handle);
    }

    for (sound_type, path) in config.sfx {
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

    storage.put_music(
        AlbumId::Ominous,
        assets.load("audio/music/ominous/main_menu_theme.ogg"),
        "Main Menu Theme".to_string(),
    );
    storage.put_music(
        AlbumId::Jazz,
        assets.load("audio/music/jazz/rustlin_in_the_pack.ogg"),
        "Rustlin' in the Pack".to_string(),
    );
    storage.put_music(
        AlbumId::Jazz,
        assets.load("audio/music/jazz/bobbin_backpack_goblin.ogg"),
        "Bobbin' Backpack Goblin".to_string(),
    );
    storage.put_music(
        AlbumId::Jazz,
        assets.load("audio/music/jazz/infernal_infamous_imp.ogg"),
        "Infernal Infamous Imp".to_string(),
    );
}

pub fn check_load_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    storage: Res<AssetStorage>,
) {
    match asset_server.get_group_load_state(storage.get_all_handle_ids()) {
        LoadState::Failed => {
            error!("Failed loading assets!");
        }
        LoadState::Loaded => {
            commands.insert_resource(NextState(AppState::MainMenu));
        }
        _ => (),
    }
}

pub fn add_configs(
    mut commands: Commands,
    assets: Res<AssetStorage>,
    audio: Res<Assets<AudioConfig>>,
    debug: Res<Assets<DebugConfig>>,
) {
    commands.insert_resource(
        audio
            .get(&assets.audio)
            .cloned()
            .expect("audio.ron wasn't loaded (yet)!"),
    );
    commands.insert_resource(
        debug
            .get(&assets.debug)
            .cloned()
            .expect("debug.ron wasn't loaded (yet)!"),
    );
}
