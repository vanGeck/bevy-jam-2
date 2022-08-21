use bevy::prelude::*;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};

use crate::config::config_debug::DebugConfig;
use crate::config::config_grid::GridConfig;
use crate::AppState;

pub struct ConfigLoadingPlugin;

impl Plugin for ConfigLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::ConfigLoading,
            ConditionSet::new()
                .run_in_state(AppState::ConfigLoading)
                .with_system(configure_ui_look)
                .with_system(load_configs)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::ConfigLoading)
                .with_system(check_load_state)
                .into(),
        );
    }
}

// This is a global look for egui
fn configure_ui_look(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn load_configs(mut commands: Commands) {
    commands.insert_resource(GridConfig::load_from_file());
    commands.insert_resource(DebugConfig::load_from_file());
}

fn check_load_state(mut commands: Commands, config: Res<DebugConfig>) {
    if config.skip_straight_to_game {
        commands.insert_resource(NextState(AppState::InGame));
    } else {
        commands.insert_resource(NextState(AppState::MainMenu));
    }
}
