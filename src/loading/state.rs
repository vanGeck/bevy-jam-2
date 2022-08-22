use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::game::AssetStorage;
use crate::loading::systems::{check_load_state, configure_ui_look, load_assets, load_configs};
use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetStorage>()
            .add_enter_system_set(
                AppState::Loading,
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .with_system(load_assets)
                    .with_system(load_configs)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .with_system(check_load_state)
                    .into(),
            )
            .add_exit_system_set(
                AppState::Loading,
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .with_system(configure_ui_look)
                    .into(),
            );
    }
}
