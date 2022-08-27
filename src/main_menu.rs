use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;

use crate::mouse::MouseInteractive;
use crate::{AppState, DebugConfig};
use crate::transition_state::MenuTransition;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::MainMenu,
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(check_menu_bypass)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(track_backpack_hover)
                .into(),
        )
        .add_exit_system_set(
            AppState::MainMenu,
            ConditionSet::new().run_in_state(AppState::MainMenu).into(),
        );
    }
}

pub fn check_menu_bypass(
    mut commands: Commands,
    mut query: Query<&mut MenuBackpack>,
    mut config: ResMut<DebugConfig>,
) {
    if config.skip_straight_to_game {
        config.skip_straight_to_game = false;
        query.single_mut().transition = MenuTransition::InactiveGame;
        commands.insert_resource(NextState(AppState::InGame));
    }
}

#[derive(Component, Default)]
pub struct MenuBackpack {
    pub transition: MenuTransition,
}

#[derive(Component)]
pub struct Backpack {
    dimens: Vec2,
}

#[derive(Component)]
pub struct BackpackFlap {
    dimens: Vec2,
    height: f32,
}

pub fn track_backpack_hover(
    mut commands: Commands,
    mut query_backpack: Query<(
        &mut MenuBackpack,
        &mut TextureAtlasSprite,
        &MouseInteractive,
    )>,
) {
    if let Ok((mut backpack, mut sprite, interactive)) = query_backpack.get_single_mut() {
        if interactive.clicked {
            commands.insert_resource(NextState(AppState::Transition));
            backpack.transition = MenuTransition::menu_to_game();
            // TODO: Maybe sound effect?
        }
        if matches!(backpack.transition, MenuTransition::InactiveMenu) {
            sprite.index = if interactive.hovered { 1 } else { 0 };
        }
    }
}
