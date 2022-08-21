use crate::*;
use crate::{
    grid::{coords::Coords, dimens::Dimens, pos::Pos},
    *,
};
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::NextState;

pub mod assets;
mod components;
mod create_grid_system;
mod draw_grid_system;
pub mod player;
mod spawn_item_system;

use crate::audio::sound_event::SoundEvent;
use crate::game::SpriteType::Croissant;
pub use assets::*;
pub use components::*;
pub use create_grid_system::*;
pub use draw_grid_system::*;
pub use player::*;
pub use spawn_item_system::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::InGame,
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(setup)
                .with_system(create_grid_system)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(draw_win_lose_placeholder_menu)
                .into(),
        );

        app.add_exit_system_set(
            AppState::InGame,
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(despawn_gameplay_entities)
                .into(),
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

fn setup(mut cmd: Commands, assets: Res<AssetStorage>, mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::Music(Some((MusicType::Placeholder, false))));
    cmd.spawn_bundle(Camera2dBundle::default())
        .insert(input::GameCamera)
        .insert(CleanupOnGameplayEnd);

    // Remove this spawn later
    spawn_item(
        &mut cmd,
        Item {
            name: "Croissant".to_string(),
            coords: Coords::new(Pos::new(2, 2), Dimens::new(3, 2)),
        },
        assets.texture(&Croissant),
    )
}

fn draw_win_lose_placeholder_menu(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    mut egui_context: ResMut<EguiContext>,
    mut result: ResMut<State<GameResult>>,
) {
    egui::Window::new("Gameplay").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Win").clicked() {
            audio.send(SoundEvent::Sfx(SoundType::Placeholder));
            commands.insert_resource(NextState(AppState::GameEnded));
            result.replace(GameResult::Won).ok();
        }
        if ui.button("Lose").clicked() {
            audio.send(SoundEvent::Sfx(SoundType::Placeholder));
            commands.insert_resource(NextState(AppState::GameEnded));
            result.replace(GameResult::Lost).ok();
        }
    });
}

pub fn despawn_gameplay_entities(
    mut cmd: Commands,
    mut audio: EventWriter<SoundEvent>,
    q: Query<Entity, With<CleanupOnGameplayEnd>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
    audio.send(SoundEvent::KillAllMusic);
}
