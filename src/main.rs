#![forbid(unsafe_code)]

use bevy::log::Level;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use mouse::{setup_mouse, update_mouse_pos};

use crate::audio::plugin::MyAudioPlugin;
use crate::config::config_debug::DebugConfig;
use crate::debug_window::DebugWindowPlugin;
use crate::game::camera::set_cam_scale;
use crate::game::GamePlugin;
use crate::game_ended::GameEndedPlugin;
use crate::loading::state::LoadingPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::states::{handle_escape, log_state_changes, AppState};
use crate::window_event_handler::handle_window;

pub mod animation;
mod audio;
mod config;
mod debug_window;
mod game;
mod game_ended;
mod hud;
mod loading;
mod main_menu;
mod mouse;
mod positioning;
mod states;
mod window_event_handler;

pub const GAME_NAME: &str = "Bag Goblin";

fn main() {
    let config = DebugConfig::load_from_file();
    let mut app = App::new();
    app.insert_resource(bevy::log::LogSettings {
        filter: config.log_filter.clone(),
        level: Level::TRACE,
        ..Default::default()
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
    .add_plugin(LoadingPlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(GamePlugin)
    .add_plugin(GameEndedPlugin)
    .add_system(handle_window)
    .add_system(log_state_changes)
    .add_system(handle_escape)
    .add_system(set_cam_scale)
    .add_startup_system(setup_mouse)
    .add_system(update_mouse_pos);
    if config.show_debug_window {
        app.add_plugin(DebugWindowPlugin);
    }
    app.insert_resource(config).run();
}
