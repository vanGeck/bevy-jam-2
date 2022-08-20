mod mainmenu;
mod game;
mod cleanup;
mod gameover;

use bevy::DefaultPlugins;
use egui::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::prelude::*;
use bevy::window::WindowId;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::game::GamePlugin;
use crate::gameover::GameOverPlugin;
use crate::mainmenu::MainMenuPlugin;

pub const GAME_NAME: &str = "Bevy Jam 2 Game";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum AppState { // changing this state alone will make game state plugins act according to new state, nothing else is needed
    AssetLoading,
    MainMenu,
    InGame,
    GameEnded
}

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
        })
        .add_loading_state(LoadingState::new(AppState::AssetLoading) // <- we load all assets in this state
             .continue_to_state(AppState::MainMenu) // and then switch to main menu
             .with_collection::<game::assets::AssetHandles>())
        .add_state(AppState::AssetLoading)
        .add_state(game::GameResult::Won)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        
        .add_startup_system(configure_ui_look)
        .run();
}

// This is a global look for egui
fn configure_ui_look(mut egui_ctx: ResMut<EguiContext>){
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}