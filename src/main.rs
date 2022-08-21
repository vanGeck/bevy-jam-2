use crate::config::config_log::LogConfig;
use crate::config_loading::ConfigLoadingPlugin;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::window::WindowMode;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use egui::*;
use heron::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::game::GamePlugin;
use crate::gameover::GameOverPlugin;
use crate::input::InputsPlugin;
use crate::mainmenu::MainMenuPlugin;
use crate::window_event_handler::handle_window;

mod cleanup;
mod config;
mod config_loading;
mod game;
mod gameover;
mod grid;
mod input;
mod mainmenu;
mod window_event_handler;

pub const GAME_NAME: &str = "Bevy Jam 2 Game";

/// Changing this state alone will make game state plugins act according to new state, nothing
/// else is needed.
///
/// Note that we are using `iyes_loopless`, the way to change states is by adding a resource:
/// `commands.insert_resource(NextState(AppState::InGame))`
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum AppState {
    AssetLoading,
    /// Load config files. It doesn't look like this can be done with a custom AssetLoader
    /// (because the file location is uncertain), and I don't know how to integrate it with the
    /// bevy asset loader. That's why I created a separate 2nd loading state.
    ConfigLoading,
    MainMenu,
    InGame,
    GameEnded,
}

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: LogConfig::load_from_file().level.parse().unwrap(),
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_loopless_state(AppState::AssetLoading)
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading) // <- we load all assets in this state
                .continue_to_state(AppState::ConfigLoading) // and then switch to the config loading state.
                .with_collection::<game::assets::AssetHandles>(),
        )
        .add_state(game::GameResult::Won)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ConfigLoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(InputsPlugin)
        .add_system(handle_window)
        .run();
}

// Define your physics layers
#[derive(PhysicsLayer)]
enum PhysLayer {
    World,
    Draggables,
}
