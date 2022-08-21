mod cleanup;
mod game;
mod gameover;
mod input;
mod mainmenu;

use crate::game::GamePlugin;
use crate::gameover::GameOverPlugin;
use crate::input::{
    move_dragged_item, process_drag_attempt, process_drag_end, process_fresh_drag, InputsPlugin,
    MousePosition,
};
use crate::mainmenu::MainMenuPlugin;
use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::DefaultPlugins;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use egui::*;
use heron::prelude::*;
mod grid;

pub const GAME_NAME: &str = "Bevy Jam 2 Game";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum AppState {
    // changing this state alone will make game state plugins act according to new state, nothing else is needed
    AssetLoading,
    MainMenu,
    InGame,
    GameEnded,
}

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading) // <- we load all assets in this state
                .continue_to_state(AppState::MainMenu) // and then switch to main menu
                .with_collection::<game::assets::AssetHandles>(),
        )
        .add_state(AppState::AssetLoading)
        .add_state(game::GameResult::Won)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(InputsPlugin)
        .add_startup_system(configure_ui_look)
        .run();
}

// This is a global look for egui
fn configure_ui_look(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

// Define your physics layers
#[derive(PhysicsLayer)]
enum PhysLayer {
    World,
    Draggables,
}
