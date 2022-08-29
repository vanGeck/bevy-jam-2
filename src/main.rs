#![forbid(unsafe_code)]
#![allow(dead_code)]

extern crate core;

use bevy::DefaultPlugins;
use bevy::log::Level;
use bevy::prelude::*;
use bevy::prelude::CoreStage::Update;
use bevy::window::WindowMode;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::audio::plugin::MyAudioPlugin;
use crate::config::config_audio::{AudioConfig, AudioConfigLoader, };
use crate::config::config_debug::{DebugConfig, DebugConfigLoader};
use crate::game::{ GamePlugin};
use crate::game::camera::set_cam_scale;
use crate::game_ended::GameEndedPlugin;
use crate::loading::state::LoadingPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::mouse::{Mouse, MousePlugin};
use crate::states::{AppState, handle_escape, log_state_changes};
use crate::transition_state::TransitionPlugin;
use crate::window_event_handler::handle_window;

pub mod animation;
mod audio;
mod config;
pub mod game;
mod game_ended;
mod hud;
mod loading;
mod main_menu;
mod mouse;
mod positioning;
mod states;
mod transition_state;
mod window_event_handler;

pub const GAME_NAME: &str = "Bag Goblin";

fn main() {
    App::new().insert_resource(bevy::log::LogSettings {
        filter: "info,wgpu=error,symphonia_core=warn,symphonia_format_ogg=warn,symphonia_codec_vorbis=warn,symphonia_bundle_mp3=warn,bag_goblin=info".to_string(),
        level: Level::TRACE,
    })
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.9)))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            resizable: true,
            ..default()
        })
        .add_loopless_state(AppState::Loading)
        .add_state(game::GameResult::Won)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MyAudioPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(TransitionPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameEndedPlugin)
        .add_asset::<AudioConfig>()
        .init_asset_loader::<AudioConfigLoader>()
        .add_asset::<DebugConfig>()
        .init_asset_loader::<DebugConfigLoader>()
        .add_system(handle_window)
        .add_system(log_state_changes)
        .add_system(handle_escape)
        .add_system(set_cam_scale)
        .run();
}
