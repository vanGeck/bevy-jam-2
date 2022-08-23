use bevy::log::Level;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use egui::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::audio::plugin::MyAudioPlugin;
use crate::config::config_log::LogConfig;
use crate::game::GamePlugin;
use crate::game_ended::GameEndedPlugin;
use crate::loading::state::LoadingPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::states::{handle_escape, log_state_changes, AppState};
use crate::window_event_handler::handle_window;

mod audio;
mod config;
mod game;
mod game_ended;
mod hud;
mod loading;
mod main_menu;
mod mouse;
mod positioning;
mod states;
mod window_event_handler;

pub const GAME_NAME: &str = "Bevy Jam 2 Game";

fn main() {
    let log_settings = LogConfig::load_from_file();
    App::new()
        .insert_resource(bevy::log::LogSettings {
            filter: log_settings.filter,
            level: Level::TRACE,
            ..Default::default()
        })
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.9)))
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            ..default()
        })
        .add_loopless_state(AppState::Loading)
        .add_state(game::GameResult::Won)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MyAudioPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameEndedPlugin)
        .add_system(handle_window)
        .add_system(log_state_changes)
        .add_system(handle_escape)
        .run();
}
