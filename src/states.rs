use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::CurrentState;
use iyes_loopless::state::NextState;

/// Changing this state alone will make game state plugins act according to new state, nothing
/// else is needed.
///
/// Note that we are using `iyes_loopless`, the way to change states is by adding a resource:
/// `commands.insert_resource(NextState(AppState::InGame))`
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum AppState {
    AssetLoading,
    /// Load config files. It doesn't look like this can be done with a custom AssetLoader
    /// (because the file location is uncertain), and I don't know how to integrate it with the
    /// bevy asset loader. That's why I created a separate 2nd loading state.
    ConfigLoading,
    MainMenu,
    InGame,
    GameEnded,
}

pub fn log_state_changes(state: Res<CurrentState<AppState>>) {
    if state.is_changed() {
        info!("Switching to game state {:?}!", state.0);
    }
}

pub fn handle_escape(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    state: Res<CurrentState<AppState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.clear_just_pressed(KeyCode::Escape) {
        match state.0 {
            AppState::AssetLoading | AppState::ConfigLoading | AppState::MainMenu => {
                exit.send(AppExit)
            }
            AppState::InGame | AppState::GameEnded => {
                commands.insert_resource(NextState(AppState::MainMenu))
            }
        }
    }
}
