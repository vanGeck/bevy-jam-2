use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::CurrentState;
use iyes_loopless::state::NextState;

use crate::main_menu::MenuBackpack;
use crate::transition_state::MenuTransition;

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
    /// Transitioning between MainMenu and InGame.
    Transition,
    InGame,
    GameEnded,
}

pub fn delete_all_entities(mut commands: Commands, query: Query<Entity>) {
    info!("Deleting all entities...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn log_state_changes(state: Res<CurrentState<AppState>>) {
    if state.is_changed() {
        info!("Switching to game state {:?}!", state.0);
    }
}

pub fn handle_escape(
    mut commands: Commands,
    mut query: Query<&mut MenuBackpack>,
    mut exit: EventWriter<AppExit>,
    state: Res<CurrentState<AppState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.clear_just_pressed(KeyCode::Escape) {
        match state.0 {
            AppState::MainMenu => {
                exit.send(AppExit);
            }
            AppState::Transition => {
                let mut menu = query.single_mut();
                menu.transition = MenuTransition::InactiveMenu;
                commands.insert_resource(NextState(AppState::MainMenu));
            }
            AppState::InGame | AppState::GameEnded => {
                let mut menu = query.single_mut();
                menu.transition = MenuTransition::game_to_menu();
                commands.insert_resource(NextState(AppState::Transition));
            }
            _ => (),
        }
    }
}
