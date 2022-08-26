use crate::game::GameCamera;
use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::CurrentState;
use iyes_loopless::state::NextState;

use crate::main_menu::MenuBackpack;

/// Changing this state alone will make game state plugins act according to new state, nothing
/// else is needed.
///
/// Note that we are using `iyes_loopless`, the way to change states is by adding a resource:
/// `commands.insert_resource(NextState(AppState::InGame))`
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum AppState {
    Loading,
    MainMenu,
    InGame,
    GameEnded,
}

pub fn log_state_changes(state: Res<CurrentState<AppState>>) {
    if state.is_changed() {
        info!("Switching to game state {:?}!", state.0);
    }
}

pub fn delete_all_entities(mut commands: Commands, query: Query<Entity>) {
    info!("Deleting all entities...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_escape(
    mut query: Query<&mut MenuBackpack>,
    mut exit: EventWriter<AppExit>,
    state: Res<CurrentState<AppState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.clear_just_pressed(KeyCode::Escape) {
        match state.0 {
            AppState::Loading | AppState::MainMenu => exit.send(AppExit),
            AppState::InGame | AppState::GameEnded => {
                query.single_mut().transition = MenuTransition::game_to_menu();
            }
        }
    }
}

pub enum MenuTransition {
    Inactive,
    FromMenuToGame(Timer),
    FromGameToMenu(Timer),
}

impl Default for MenuTransition {
    fn default() -> Self {
        Self::Inactive
    }
}

impl MenuTransition {
    pub fn menu_to_game() -> Self {
        MenuTransition::FromMenuToGame(Timer::from_seconds(2., false))
    }
    pub fn game_to_menu() -> Self {
        MenuTransition::FromGameToMenu(Timer::from_seconds(2., false))
    }
}

/// Animates the transition from the main menu into the game.
pub fn handle_state_transition(
    mut commands: Commands,
    time: Res<Time>,
    mut query_pack: Query<(&mut MenuBackpack, &mut TextureAtlasSprite)>,
    mut query_cam: Query<&mut GameCamera>,
) {
    let (mut menu, mut sprite) = query_pack.single_mut();
    let mut cam = query_cam.single_mut();
    match &mut menu.transition {
        MenuTransition::FromMenuToGame(timer) => {
            timer.tick(time.delta());
            sprite.index = 1;
            sprite.color = Color::rgba(1., 1., 1., timer.percent_left());
            cam.zoom = cam.game_zoom + (cam.menu_zoom - cam.game_zoom) * timer.percent_left();
            if timer.finished() {
                commands.insert_resource(NextState(AppState::InGame));
                menu.transition = MenuTransition::Inactive;
            }
        }
        MenuTransition::FromGameToMenu(timer) => {
            timer.tick(time.delta());
            sprite.index = 1;
            sprite.color = Color::rgba(1., 1., 1., 1. - timer.percent_left());
            cam.zoom =
                cam.game_zoom + (cam.menu_zoom - cam.game_zoom) * (1. - timer.percent_left());
            if timer.finished() {
                commands.insert_resource(NextState(AppState::MainMenu));
                menu.transition = MenuTransition::Inactive;
            }
        }
        _ => (),
    }
}
