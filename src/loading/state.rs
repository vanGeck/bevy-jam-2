use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::game::{
    create_camera, create_layout_background, create_layout_feed, create_layout_foo,
    create_layout_grids, create_layout_hero, create_layout_music, create_layout_toasts,
    AssetStorage,
};
use crate::hud::gold::setup_gold;
use crate::loading::systems::{check_load_state, load_assets, load_configs};
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
                    .with_system(create_camera)
                    .with_system(create_layout_background)
                    .with_system(create_layout_music)
                    .with_system(create_layout_feed)
                    .with_system(create_layout_grids)
                    .with_system(create_layout_toasts)
                    .with_system(create_layout_foo)
                    .with_system(create_layout_hero)
                    .with_system(setup_gold)
                    .into(),
            );
    }
}
