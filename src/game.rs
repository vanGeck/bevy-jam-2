use crate::{*, grid::{coords::Coords, pos::Pos, dimens::Dimens}};

pub mod assets;
mod components;
mod create_grid_system;
mod create_items_system;
mod draw_grid_system;
pub mod player;

pub use assets::*;
pub use components::*;
pub use create_grid_system::*;
pub use create_items_system::*;
pub use draw_grid_system::*;
pub use player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup)
                .with_system(create_grid_system)
        );

        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(draw_win_lose_placeholder_menu), // .with_system(rotate_player_placeholder)
        );

        app.add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(despawn_gameplay_entities),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum GameResult {
    Lost,
    Won,
}

// Place this component on every gameplay entity that needs to be destroyed when game ends.
#[derive(Component)]
pub struct CleanupOnGameplayEnd;

fn setup(mut cmd: Commands, assets: Res<AssetHandles>) { 
    cmd.spawn_bundle(Camera2dBundle::default())
        .insert(input::GameCamera)
        .insert(CleanupOnGameplayEnd);
    
    spawn_item(cmd, Item{
        name: "Croissant".to_string(),
        coords: Coords::new(Pos::new(2, 2), Dimens::new(3, 2)),
        occupied: vec![vec![true, true, true], vec![true, true, true]]
        },
        assets.three_x_two_croissant.clone(),
    )
}

fn draw_win_lose_placeholder_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut result: ResMut<State<GameResult>>,
) {
    egui::Window::new("Gameplay").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Win").clicked() {
            state.replace(AppState::GameEnded).ok();
            result.replace(GameResult::Won).ok();
        }
        if ui.button("Lose").clicked() {
            state.replace(AppState::GameEnded).ok();
            result.replace(GameResult::Lost).ok();
        }
    });
}

pub fn despawn_gameplay_entities(mut cmd: Commands, q: Query<Entity, With<CleanupOnGameplayEnd>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}
