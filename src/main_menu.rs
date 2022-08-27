use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem};
use iyes_loopless::state::NextState;

use crate::game::create_backpack::create_layout_background;
use crate::game::create_camera;
use crate::game::create_widget_feed::create_layout_feed;
use crate::game::create_widget_grids::{create_layout_foo, create_layout_grids};
use crate::game::create_widget_hero::create_layout_hero;
use crate::game::create_widget_music::create_layout_music;
use crate::game::create_widget_toasts::create_layout_toasts;
use crate::hud::gold::setup_gold;
use crate::mouse::MouseInteractive;
use crate::states::delete_all_entities;
use crate::transition_state::MenuTransition;
use crate::{AppState, DebugConfig};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::MainMenu,
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(delete_all_entities)
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
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(check_menu_bypass.run_if(should_check_bypass))
                .with_system(track_backpack_hover)
                .into(),
        )
        .add_exit_system_set(
            AppState::MainMenu,
            ConditionSet::new().run_in_state(AppState::MainMenu).into(),
        );
    }
}

pub fn should_check_bypass(config: Res<DebugConfig>) -> bool {
    config.skip_straight_to_game
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
