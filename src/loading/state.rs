use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::game::AssetStorage;
use crate::loading::systems::{add_configs, check_load_state, load_assets, load_configs};
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
                    .with_system(display_loading_message)
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
                    .with_system(add_configs)
                    .into(),
            );
    }
}

pub fn display_loading_message(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    let text_style = TextStyle {
        font: server.load("fonts/FiraSans-BoldItalic.ttf"),
        font_size: 250.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Loading...".to_string(), text_style)
            .with_alignment(text_alignment),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(Vec3::new(
            1. / 2.,
            1. / 2.,
            1.,
        )),
        ..default()
    });
}
