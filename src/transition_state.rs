use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};

use crate::{AppState, Mouse};
use crate::game::GameCamera;
use crate::main_menu::MenuBackpack;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Transition,
            ConditionSet::new()
                .run_in_state(AppState::Transition)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                // Run in any state:
                .with_system(handle_state_transition)
                .into(),
        )
        .add_exit_system_set(
            AppState::Transition,
            ConditionSet::new()
                .run_in_state(AppState::Transition)
                .into(),
        );
    }
}


pub enum MenuTransition {
    InactiveMenu,
    InactiveGame,
    FromMenuToGame(Timer),
    FromGameToMenu(Timer),
}

impl Default for MenuTransition {
    fn default() -> Self {
        Self::InactiveMenu
    }
}

impl MenuTransition {
    pub fn menu_to_game() -> Self {
        MenuTransition::FromMenuToGame(Timer::from_seconds(1.5, false))
    }
    pub fn game_to_menu() -> Self {
        MenuTransition::FromGameToMenu(Timer::from_seconds(0.5, false))
    }
}

/// Animates the transition from the main menu into the game.
pub fn handle_state_transition(
    mut commands: Commands,
    mut mouse: ResMut<Mouse>,
    time: Res<Time>,
    mut query_pack: Query<(&mut MenuBackpack, &mut TextureAtlasSprite, &mut Visibility)>,
    mut query_cam: Query<&mut GameCamera>,
) {
    if let Ok((mut menu, mut sprite, mut visibility)) = query_pack.get_single_mut() {
        let mut cam = query_cam.single_mut();
        match &mut menu.transition {
            MenuTransition::FromMenuToGame(timer) => {
                timer.tick(time.delta());
                sprite.index = 1;
                mouse.disabled = true;
                visibility.is_visible = true;
                let progress = 1. - timer.percent().powi(2);
                sprite.color = Color::rgba(1., 1., 1., progress);
                cam.zoom = cam.game_zoom + (cam.menu_zoom - cam.game_zoom) * progress;
                if timer.finished() {
                    commands.insert_resource(NextState(AppState::InGame));
                    menu.transition = MenuTransition::InactiveGame;
                }
            }
            MenuTransition::FromGameToMenu(timer) => {
                timer.tick(time.delta());
                sprite.index = 1;
                mouse.disabled = true;
                visibility.is_visible = true;
                let progress = 1. - timer.percent_left();
                sprite.color = Color::rgba(1., 1., 1., progress);
                cam.zoom = cam.game_zoom + (cam.menu_zoom - cam.game_zoom) * progress;
                if timer.finished() {
                    commands.insert_resource(NextState(AppState::MainMenu));
                    menu.transition = MenuTransition::InactiveMenu;
                }
            }
            MenuTransition::InactiveMenu => {
                mouse.disabled = false;
                visibility.is_visible = true;
                sprite.color = Color::rgba(1., 1., 1., 1.);
                cam.zoom = cam.menu_zoom;
            }
            MenuTransition::InactiveGame => {
                mouse.disabled = false;
                visibility.is_visible = false;
                sprite.color = Color::rgba(1., 1., 1., 0.);
                cam.zoom = cam.game_zoom;
            }
        }
    }
}
